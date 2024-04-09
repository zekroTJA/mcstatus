use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct ErrorResponse(anyhow::Error);

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for ErrorResponse
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
