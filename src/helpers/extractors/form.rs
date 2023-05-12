use crate::error::Error;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{Form, FromRequest, Multipart},
    http::Request,
    response::{IntoResponse, Response},
    BoxError,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use validator::Validate;

#[async_trait]
pub trait FromMultipart: Sized + Send + Sync {
    async fn from_multipart(multipart: Multipart) -> Result<Self, Error>;
}

pub struct MultipartFrom<T: FromMultipart>(pub T);

pub struct MultipartFormWithValidation<T: Validate + FromMultipart>(pub T);

pub struct FormWithValidation<T: Validate>(pub T);

#[derive(Debug, Clone)]
pub struct FileField {
    pub file_name: String,
    pub content_type: String,
    pub file: Bytes,
    pub file_extension: String,
    pub size: usize,
}

impl FileField {
    pub fn new(file_name: String, content_type: String, file: Bytes) -> Self {
        let size = file.len();
        // TODO validate file extension
        let file_extension = match file_name.split('.').last() {
            Some(ext) => ext.to_string(),
            None => String::from(""),
        };

        Self {
            file_name,
            content_type,
            file,
            file_extension,
            size,
        }
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for FormWithValidation<T>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,

    T: Validate + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(data): Form<T> = match Form::from_request(req, state).await {
            Ok(data) => data,
            Err(e) => {
                return Err(Error::FormError(e));
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

#[async_trait]
impl<S, B, T> FromRequest<S, B> for MultipartFrom<T>
where
    B::Data: Into<Bytes>,
    B: HttpBody + Send + 'static,
    B::Error: Into<BoxError>,

    T: FromMultipart,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let multipart = Multipart::from_request(req, state)
            .await
            // the only possible e is Invalid boundry
            .map_err(|e| e.into_response())?;

        Ok(MultipartFrom(
            T::from_multipart(multipart)
                .await
                .map_err(|e| e.into_response())?,
        ))
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for MultipartFormWithValidation<T>
where
    B::Data: Into<Bytes>,
    B: HttpBody + Send + 'static,
    B::Error: Into<BoxError>,

    T: FromMultipart + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let MultipartFrom(data) = MultipartFrom::<T>::from_request(req, state).await?;

        data.validate()
            .map_err(|e| Error::StructValidation(e).into_response())?;

        Ok(MultipartFormWithValidation(data))
    }
}
