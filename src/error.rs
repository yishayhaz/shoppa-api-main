use crate::helpers::types::ResponseBuilder;
use axum::extract::multipart::MultipartError;
use axum::response::{IntoResponse, Response};
use mongodb::error::{ErrorKind, WriteFailure};
use validator::ValidationErrors;
// Main Crate Error

#[derive(Debug)]
pub enum Error {
    /// For starter, to remove as code matures.
    Generic(String),
    /// For starter, to remove as code matures.
    Static(&'static str),
    // The string is for the collection name.
    DBError((&'static str, mongodb::error::Error)),
    // The string is for the collection name.
    NoEntityId(&'static str),
    HashError(argon2::password_hash::Error),
    Serilaztion,
    StructValidation(ValidationErrors),
    MultiPartFormError(MultipartError),
    FileUploadError(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Generic(e) => {
                ResponseBuilder::<u16>::error("", None, Some(e.as_str()), Some(500)).into_response()
            }
            Self::Static(e) => {
                ResponseBuilder::<u16>::error("", None, Some(e), Some(500)).into_response()
            }
            Self::DBError(e) => match *e.1.kind {
                // Here we catch a duplicate key error and return a 409
                ErrorKind::Write(we) => match we {
                    WriteFailure::WriteError(wf) => {
                        if wf.code == 11000 {
                            return ResponseBuilder::<u16>::error(
                                "",
                                None,
                                Some("Document alredy exists"),
                                Some(409),
                            )
                            .into_response();
                        } else {
                            ResponseBuilder::error(
                                "",
                                Some(wf.message.as_str()),
                                Some(e.0),
                                Some(500),
                            )
                            .into_response()
                        }
                    }
                    WriteFailure::WriteConcernError(wc) => {
                        ResponseBuilder::error("", Some(wc.message.as_str()), Some(e.0), Some(500))
                            .into_response()
                    }
                    _ => ResponseBuilder::error(
                        "",
                        Some("mongodb added error we need to handle"),
                        Some(e.0),
                        Some(500),
                    )
                    .into_response(),
                },
                // all other errors are considered unknown
                _ => {
                    ResponseBuilder::error("", Some(e.1.to_string().as_str()), Some(e.0), Some(500))
                        .into_response()
                }
            },
            Self::NoEntityId(e) => {
                ResponseBuilder::error("", Some("No Entity ID"), Some(e), Some(500)).into_response()
            }
            Self::HashError(e) => ResponseBuilder::error(
                "",
                Some(e.to_string().as_str()),
                Some("Hash Error"),
                Some(500),
            )
            .into_response(),
            Self::Serilaztion => ResponseBuilder::error(
                "",
                Some("Serilaztion Error"),
                Some("Serilaztion Error"),
                Some(500),
            )
            .into_response(),
            Self::StructValidation(e) => {
                ResponseBuilder::validation_error(Some(e), None).into_response()
            }
            Self::MultiPartFormError(e) => ResponseBuilder::error(
                "",
                Some(e.to_string().as_str()),
                Some("MultiPartForm Error"),
                Some(500),
            )
            .into_response(),
            Self::FileUploadError(e) => ResponseBuilder::error(
                "",
                Some(e.to_string().as_str()),
                Some("File Upload Error"),
                Some(500),
            )
            .into_response(),
        }
    }
}
