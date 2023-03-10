use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::{Request, StatusCode},
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
        // TODO: improve the error
        let body = match self {
            Self::JsonError(e) => e.to_string(),
            Self::JsonValidation(e) => {
                e.errors()
                    .iter()
                    .map(|(k, v)| format!("{}: {:?}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            },
        };
        (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
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
