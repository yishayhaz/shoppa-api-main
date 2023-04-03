use super::super::types::ResponseBuilder;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

pub enum QueryValidationError {
    ParseError,
    Validation(ValidationErrors),
}

impl IntoResponse for QueryValidationError {
    fn into_response(self) -> Response {
        match self {
            Self::ParseError => ResponseBuilder::<u16>::validation_error(
                None,
                Some("Missing some required query params"),
            )
            .into_response(),
            Self::Validation(e) => ResponseBuilder::validation_error(
                Some(e),
                Some("Validation error for query params"),
            )
            .into_response(),
        }
    }
}

pub struct QueryWithValidation<T: Validate>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for QueryWithValidation<T>
where
    T: Validate + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = QueryValidationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(v) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|_| QueryValidationError::ParseError)?;

        v.validate()
            .map_err(|op| QueryValidationError::Validation(op))?;

        Ok(QueryWithValidation(v))
    }
}
