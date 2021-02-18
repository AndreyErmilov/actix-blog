use crate::error::ServiceError;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ErrorPayload {
    error: String,
    message: Option<String>,
}

impl From<&ServiceError> for ErrorPayload {
    fn from(error: &ServiceError) -> Self {
        Self {
            error: error.to_string(),
            message: match error {
                ServiceError::DatabaseError(err) => Some(err.to_string()),
                ServiceError::MailboxError(err) => Some(err.to_string()),
            },
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let payload = ErrorPayload::from(self);
        HttpResponse::InternalServerError().json(payload)
    }
}
