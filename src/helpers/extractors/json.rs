use super::super::types::ResponseBuilder;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
    response::{IntoResponse, Response},
    BoxError,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

pub struct JsonWithValidation<T: Validate>(pub T);
pub enum JsonValidationError {
    JsonError(JsonRejection),
    JsonValidation(ValidationErrors),
}

impl IntoResponse for JsonValidationError {
    fn into_response(self) -> Response {
        match self {
            Self::JsonError(e) => match e {
                JsonRejection::BytesRejection(e) => ResponseBuilder::validation_error(
                    Some(e.to_string()),
                    Some("bytes error"),
                )
                .into_response(),
                JsonRejection::JsonSyntaxError(e) => ResponseBuilder::validation_error(
                    Some(e.to_string()),
                    Some("deserialize error"),
                )
                .into_response(),
                JsonRejection::MissingJsonContentType(e) => ResponseBuilder::validation_error(
                    Some(e.to_string()),
                    Some("content type error"),
                )
                .into_response(),
                JsonRejection::JsonDataError(e) => ResponseBuilder::validation_error(
                    Some(e.to_string()),
                    Some("missing header error"),
                )
                .into_response(),
                _ => ResponseBuilder::error(
                    // TODO add error code here
                    "",
                    Some(e.to_string()),
                    Some("unknown error"),
                    Some(500),
                )
                .into_response(),
            },
            Self::JsonValidation(e) => {
                ResponseBuilder::validation_error(Some(e), None).into_response()
            }
        }
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonWithValidation<T>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,

    T: Validate + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = JsonValidationError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data): Json<T> = match Json::from_request(req, state).await {
            Ok(data) => data,
            Err(e) => {
                return Err(JsonValidationError::JsonError(e));
            }
        };
        match data.validate() {
            Ok(_) => {
                return Ok(Self(data));
            }
            Err(e) => {
                return Err(JsonValidationError::JsonValidation(e));
            }
        }
    }
}
