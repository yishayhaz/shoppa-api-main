use crate::helpers::types::ResponseBuilder;
use axum::extract::{
    multipart::MultipartError,
    rejection::{FormRejection, JsonRejection},
};
use axum::response::{IntoResponse, Response};
use mongodb::error::{ErrorKind, WriteFailure};
use validator::ValidationErrors;
// Main Crate Error
use std::error::Error as StdError;

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
    FormError(FormRejection),
    JsonError(JsonRejection),
    FileUploadError(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>),
    NoNewDataProvided,
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
            Self::MultiPartFormError(e) => {
                let e = e.source().unwrap_or(&e);
                return ResponseBuilder::error(
                    "",
                    Some(e.to_string().as_str()),
                    Some("MultiPartForm Error"),
                    Some(500),
                )
                .into_response();
            }
            Self::FileUploadError(e) => ResponseBuilder::error(
                "",
                Some(e.to_string().as_str()),
                Some("File Upload Error"),
                Some(500),
            )
            .into_response(),
            Self::FormError(e) => {
                match e {
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
                }
            }
            Self::JsonError(e) => {
                match e {
                    JsonRejection::BytesRejection(e) => {
                        ResponseBuilder::validation_error(Some(e.to_string()), Some("bytes error"))
                            .into_response()
                    }
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
                    JsonRejection::JsonDataError(e) => {
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
                }
            },
            Self::NoNewDataProvided => ResponseBuilder::<u16>::validation_error(
                None,
                Some("No new data provided")
            ).into_response()
        }
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::HashError(e)
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Self::StructValidation(e)
    }
}

impl From<MultipartError> for Error {
    fn from(e: MultipartError) -> Self {
        Self::MultiPartFormError(e)
    }
}