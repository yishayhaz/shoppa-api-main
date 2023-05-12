use crate::error::Error;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    BoxError,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct JsonWithValidation<T: Validate>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonWithValidation<T>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,

    T: Validate + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data): Json<T> = match Json::from_request(req, state).await {
            Ok(data) => data,
            Err(e) => {
                return Err(Error::JsonError(e));
            }
        };
        match data.validate() {
            Ok(_) => {
                return Ok(Self(data));
            }
            Err(e) => {
                return Err(Error::StructValidation(e));
            }
        }
    }
}
