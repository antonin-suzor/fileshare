use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use axum_macros::FromRequest;
use serde_json::json;

#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiMessage))]
pub struct JsonExtract<T>(pub T);

pub struct ApiMessage {
    pub status: StatusCode,
    pub message: String,
}
impl ApiMessage {
    pub fn with_500(message: &str) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from(message),
        }
    }
}
impl From<StatusCode> for ApiMessage {
    fn from(value: StatusCode) -> Self {
        Self {
            status: value,
            message: value
                .canonical_reason()
                .unwrap_or("Unknown Status Code")
                .to_string(),
        }
    }
}
impl IntoResponse for ApiMessage {
    fn into_response(self) -> Response {
        (self.status, Json(json!({"message": self.message}))).into_response()
    }
}

pub fn map_err_to_500<E: ToString>(err: E) -> ApiMessage {
    ApiMessage {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: err.to_string(),
    }
}

pub fn context_to_500(err: anyhow::Error) -> ApiMessage {
    ApiMessage {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("{:#}", err),
    }
}
