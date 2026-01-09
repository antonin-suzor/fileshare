use crate::{
    dtos::{UploadResponse, UploadStartRequest, UploadStartResponse},
    services::{AuthService, UploadService},
    utils::{ApiMessage, context_to_500},
};
use anyhow::Context;
use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
};
use sqlx::PgPool;
use uuid::Uuid;

/// Controller for /api/uploads
pub struct UploadController {}
impl UploadController {
    // GET /api/uploads
    pub async fn get_api_uploads(
        State(db_pool): State<PgPool>,
    ) -> Result<Json<Vec<UploadResponse>>, ApiMessage> {
        let uploads = UploadService::list(&db_pool)
            .await
            .with_context(|| "Failed to list uploads")
            .map_err(context_to_500)?;
        Ok(Json(uploads.iter().map(|u| u.into()).collect()))
    }

    // GET /api/uploads/{id}
    pub async fn get_api_uploads_id(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
        Path(id): Path<Uuid>,
    ) -> Result<Json<UploadResponse>, ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;
        let upload_db_opt = UploadService::from_id(&db_pool, &id)
            .await
            .with_context(|| "Failed to get upload from id")
            .map_err(context_to_500)?;

        // Once we get the upload, verify access right
        match upload_db_opt {
            Some(upload_db) => {
                if let Some(upload_user_id) = upload_db.user_id
                    && upload_user_id != user.id
                {
                    Err(ApiMessage {
                        status: StatusCode::FORBIDDEN,
                        message: "This upload does not belong to you".to_string(),
                    })
                } else {
                    Ok(Json(upload_db.into()))
                }
            },
            None => Err(ApiMessage {
                status: StatusCode::NOT_FOUND,
                message: format!("no upload with id {id}"),
            }),
        }
    }

    // GET /api/uploads/mine
    pub async fn get_api_uploads_mine(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
    ) -> Result<Json<Vec<UploadResponse>>, ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;
        let uploads = UploadService::from_user_id(&db_pool, &user.id)
            .await
            .with_context(|| "Failed to get upload from user id")
            .map_err(context_to_500)?;
        Ok(Json(uploads.iter().map(|u| u.into()).collect()))
    }

    // POST /api/uploads/start
    pub async fn post_api_uploads_start(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
        Json(request): Json<UploadStartRequest>,
    ) -> Result<Json<UploadStartResponse>, ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;
        let presigned_put_response = UploadService::register_new_upload_and_generate_presigned_put(
            &db_pool,
            user,
            request.file_name,
            request.content_type,
            request.expires_at,
        )
        .await
        .with_context(|| "Failed to start process for new upload")
        .map_err(context_to_500)?;
        Ok(Json(UploadStartResponse {
            url: presigned_put_response,
        }))
    }

    /// DELETE /api/uploads/{id}
    pub async fn delete_api_uploads_id(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, ApiMessage> {
        // Verify access right
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;
        let upload_db = UploadService::from_id(&db_pool, &id)
            .await
            .with_context(|| "Failed to get upload from id")
            .map_err(context_to_500)?
            .ok_or_else(|| ApiMessage {
                status: StatusCode::NOT_FOUND,
                message: format!("no upload with id {id}"),
            })?;
        if let Some(upload_user_id) = upload_db.user_id
            && upload_user_id != user.id
        {
            return Err(ApiMessage {
                status: StatusCode::FORBIDDEN,
                message: "This upload does not belong to you".to_string(),
            });
        }

        // Delete the upload
        UploadService::delete_upload(&db_pool, upload_db)
            .await
            .with_context(|| "Failed to delete the upload")
            .map_err(context_to_500)?;

        Ok(StatusCode::NO_CONTENT)
    }

    /// Router to nest in /api/uploads
    pub fn router() -> Router<PgPool> {
        Router::new()
            .route("/", get(Self::get_api_uploads))
            .route(
                "/{id}",
                get(Self::get_api_uploads_id).delete(Self::delete_api_uploads_id),
            )
            .route("/mine", get(Self::get_api_uploads_mine))
            .route("/start", post(Self::post_api_uploads_start))
    }
}
