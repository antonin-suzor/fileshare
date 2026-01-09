use crate::{
    entities::{Upload, User},
    repositories::UploadRepository,
    services::DiscordService,
};
use anyhow::Context;
use aws_sdk_s3::{
    Client,
    config::{Builder, Credentials, Region},
    presigning::PresigningConfig,
};
use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use sqlx::{Error as SqlxError, PgPool};
use std::{env, time::Duration};
use uuid::Uuid;

pub struct UploadService {}
impl UploadService {
    pub async fn list(db_pool: &PgPool) -> Result<Vec<Upload>, SqlxError> {
        UploadRepository::list(db_pool).await
    }

    pub async fn from_id(db_pool: &PgPool, id: &Uuid) -> Result<Option<Upload>, SqlxError> {
        UploadRepository::from_id(db_pool, id).await
    }

    pub async fn from_user_id(db_pool: &PgPool, user_id: &Uuid) -> Result<Vec<Upload>, SqlxError> {
        UploadRepository::from_user_id(db_pool, user_id).await
    }

    pub fn get_s3_client() -> Client {
        let url = env::var("S3_URL").expect("env var S3_URL should be set");
        let key = env::var("S3_ACCESS_KEY_ID").expect("env var S3_ACCESS_KEY_ID should be set");
        let secret =
            env::var("S3_SECRET_ACCESS_KEY").expect("env var S3_SECRET_ACCESS_KEY should be set");
        let region = env::var("S3_REGION").expect("env var S3_REGION should be set");
        let path_style_buckets = env::var("S3_PATH_STYLE_BUCKETS")
            .expect("env var S3_PATH_STYLE_BUCKETS should be set")
            == "true";

        Client::from_conf(
            Builder::new()
                .endpoint_url(url)
                .credentials_provider(Credentials::new(key, secret, None, None, "yes"))
                .region(Region::new(region))
                .force_path_style(path_style_buckets)
                .build(),
        )
    }

    pub fn get_bucket_name() -> String {
        env::var("S3_BUCKET_NAME").expect("env var S3_BUCKET_NAME should be set")
    }

    pub async fn register_new_upload_and_generate_presigned_put(
        db_pool: &PgPool,
        user: User,
        file_name: String,
        content_type: String,
        expires_at: DateTime<FixedOffset>,
    ) -> anyhow::Result<String> {
        let client = Self::get_s3_client();
        let id = Uuid::new_v4();
        let obj_key = format!("content/{}/{}", id, file_name);
        let presigned_get_response = client
            .get_object()
            .bucket(Self::get_bucket_name())
            .key(&obj_key)
            .presigned(
                PresigningConfig::expires_in(
                    TimeDelta::to_std(&(expires_at - Utc::now().fixed_offset()))
                        .with_context(|| "Failed to convert expires_at to expires_in")?,
                )
                .with_context(|| "Failed to convert Duration to PresigningConfig")?,
            )
            .await
            .with_context(|| "Failed to presign get request")?;
        let presigned_get_url = String::from(presigned_get_response.uri());

        UploadRepository::insert(
            db_pool,
            &id,
            &user.id,
            &file_name,
            &content_type,
            &presigned_get_url,
            &expires_at,
        )
        .await
        .with_context(|| "Failed to create new upload in db")?;

        // Notify Discord of upload started
        let _ = DiscordService::notify_upload_started(&user.email, &file_name, &presigned_get_url)
            .await;

        Ok(String::from(
            client
                .put_object()
                .bucket(Self::get_bucket_name())
                .key(&obj_key)
                .content_type(content_type)
                .presigned(
                    PresigningConfig::expires_in(Duration::from_secs(30))
                        .with_context(|| "Failed to convert Duration to PresigningConfig")?,
                )
                .await
                .with_context(|| "Failed to presign put request")?
                .uri(),
        ))
    }

    pub async fn delete_upload(db_pool: &PgPool, upload: Upload) -> anyhow::Result<()> {
        let client = Self::get_s3_client();
        let obj_key = format!("content/{}/{}", upload.id, upload.file_name);
        client
            .delete_object()
            .bucket(Self::get_bucket_name())
            .key(obj_key)
            .send()
            .await
            .with_context(|| "Failed to delete upload in the bucket")?;
        UploadRepository::delete_from_id(db_pool, &upload.id)
            .await
            .with_context(|| "Failed to delete upload in the db")?;
        Ok(())
    }
}
