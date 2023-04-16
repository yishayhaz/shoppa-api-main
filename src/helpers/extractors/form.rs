use super::super::types::ResponseBuilder;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{rejection::FormRejection, Form, FromRequest, Multipart},
    http::Request,
    response::{IntoResponse, Response},
    BoxError,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

pub struct MultipartFrom<T: TryFrom<Multipart>>(pub T);

pub struct MultipartFormWithValidation<T: Validate + TryFrom<Multipart>>(pub T);

pub struct FormWithValidation<T: Validate>(pub T);
pub enum FormValidationError {
    FormError(FormRejection),
    FormValidation(ValidationErrors),
}

impl IntoResponse for FormValidationError {
    fn into_response(self) -> Response {
        match self {
            Self::FormError(e) => match e {
                FormRejection::BytesRejection(e) => {
                    ResponseBuilder::validation_error(Some(e.to_string()), Some("bytes error"))
                        .into_response()
                }
                FormRejection::InvalidFormContentType(e) => ResponseBuilder::validation_error(
                    Some(e.to_string()),
                    Some("content type error"),
                )
                .into_response(),
                FormRejection::FailedToDeserializeForm(e) => {
                    ResponseBuilder::validation_error(Some(e.to_string()), Some("Invalid data"))
                        .into_response()
                }
                FormRejection::FailedToDeserializeFormBody(e) => {
                    ResponseBuilder::validation_error(Some(e.to_string()), Some("Invalid data"))
                        .into_response()
                }
                _ => ResponseBuilder::error(
                    // TODO add error code here
                    "",
                    Some(e.to_string()),
                    Some("unknown error"),
                    Some(500),
                )
                .into_response(),
            },
            Self::FormValidation(e) => {
                ResponseBuilder::validation_error(Some(e), None).into_response()
            }
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
    type Rejection = FormValidationError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(data): Form<T> = match Form::from_request(req, state).await {
            Ok(data) => data,
            Err(e) => {
                return Err(FormValidationError::FormError(e));
            }
        };
        match data.validate() {
            Ok(_) => {
                return Ok(Self(data));
            }
            Err(e) => {
                return Err(FormValidationError::FormValidation(e));
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

    T: DeserializeOwned + TryFrom<Multipart>,
    T::Error: IntoResponse,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let multipart = Multipart::from_request(req, state);

        Err(().into_response())
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for MultipartFormWithValidation<T>
where
    B::Data: Into<Bytes>,
    B: HttpBody + Send + 'static,
    B::Error: Into<BoxError>,

    T: DeserializeOwned + TryFrom<Multipart> + Validate,
    T::Error: IntoResponse,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let MultipartFrom(data) = MultipartFrom::<T>::from_request(req, state).await?;

        let _ = data.validate();

        Err(().into_response())
    }
}
