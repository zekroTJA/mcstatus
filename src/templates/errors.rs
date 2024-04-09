use askama::Template;

#[derive(Template)]
#[template(path = "server-error.html")]
pub struct ServerError {
    message: String,
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        let err: anyhow::Error = value.into();
        Self {
            message: err.to_string(),
        }
    }
}
