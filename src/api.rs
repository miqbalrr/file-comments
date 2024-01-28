use axum::{
    body::Body,
    extract::FromRequest,
    http,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

pub type ApiResponse<T> = Result<AppJson<BaseResponse<T>>, AppError>;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct BaseResponse<T: Serialize> {
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T> BaseResponse<T>
where
    T: Serialize,
{
    pub fn success(data: Option<T>) -> AppJson<Self> {
        AppJson(Self {
            success: true,
            message: "success".to_owned(),
            data: data,
        })
    }
}

pub enum AppError {
    NotFound,
    InternalServerError,
    UnprocessableEntity(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            AppError::NotFound => (http::StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::InternalServerError => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::UnprocessableEntity(err_text) => {
                (http::StatusCode::UNPROCESSABLE_ENTITY, err_text)
            }
        };

        let body = Json(BaseResponse::<bool> {
            message,
            success: false,
            data: None,
        });
        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::UnprocessableEntity(err.to_string())
    }
}
