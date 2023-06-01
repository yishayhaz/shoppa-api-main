use crate::prelude::*;
use axum::{
    body::HttpBody,
    extract::{Form, FromRequest, Multipart},
    http::Request,
    response::{IntoResponse, Response},
    BoxError,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use validator::{HasLen, Validate};

#[async_trait]
pub trait FromMultipart: Sized + Send + Sync {
    async fn from_multipart(multipart: Multipart) -> Result<Self>;
}

pub struct MultipartFrom<T: FromMultipart>(pub T);

pub struct MultipartFormWithValidation<T: Validate + FromMultipart>(pub T);

pub struct FormWithValidation<T: Validate>(pub T);

// The str in the name is to allow the use of Validate(Length(min = 8)) on the file name
// I need to open a ticket on github to allow any struct that impl HasLen to be used with Validate length
#[derive(Debug, Clone, Validate)]
pub struct FileFieldstr {
    pub file_name: String,
    pub content_type: String,
    pub file: Bytes,
    pub file_extension: String,
    pub size: usize,
}

impl FileFieldstr {
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

impl HasLen for &FileFieldstr {
    fn length(&self) -> u64 {
        self.size as u64
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

    async fn from_request(req: Request<B>, state: &S) -> StdResult<Self, Self::Rejection> {
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

    async fn from_request(req: Request<B>, state: &S) -> StdResult<Self, Self::Rejection> {
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

    async fn from_request(req: Request<B>, state: &S) -> StdResult<Self, Self::Rejection> {
        let MultipartFrom(data) = MultipartFrom::<T>::from_request(req, state).await?;

        data.validate()
            .map_err(|e| Error::StructValidation(e).into_response())?;

        Ok(MultipartFormWithValidation(data))
    }
}

use serde::Serialize;

impl Serialize for FileFieldstr {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FileFieldstr", 5)?;
        state.serialize_field("file_name", &self.file_name)?;
        state.serialize_field("content_type", &self.content_type)?;
        state.serialize_field("file_extension", &self.file_extension)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("file", "file")?;
        state.end()
    }
}

#[derive(Validate, Serialize)]
struct Test2 {
    #[validate(length(min = 1))]
    test: FileFieldstr,
}
