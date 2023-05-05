use axum::response::{IntoResponse, Response};
use crate::helpers::types::ResponseBuilder;
// Main Crate Error

#[derive(Debug)]
pub enum Error {
	/// For starter, to remove as code matures.
	Generic(String),
	/// For starter, to remove as code matures.
	Static(&'static str),
    // The string is for the collection name.
    DBError((&'static str, mongodb::error::Error)),
    // DBInsertError(InsertDocumentErrors),
    // The string is for the collection name.
    NoEntityId(&'static str),
    HashError(argon2::password_hash::Error),
    Serilaztion

}

pub enum InsertDocumentErrors {
    UnknownError,
    AlredyExists,
    BsonConversionError,
    PopulatedRefField,
    InvalidArgumentsForModel,
}


impl IntoResponse for Error {

    fn into_response(self) -> Response {
        match self {
            Self::Generic(e) => {
                ResponseBuilder::<u16>::error("", None, Some(e.as_str()), Some(500)).into_response()
            },
            Self::Static(e) => {
                ResponseBuilder::<u16>::error("", None, Some(e), Some(500)).into_response()
            },
            Self::DBError(e) => {
                ResponseBuilder::error("", Some(e.1.to_string().as_str()), Some(e.0), Some(500)).into_response()
            },
            Self::NoEntityId(e) => {
                ResponseBuilder::error("", Some("No Entity ID"), Some(e), Some(500)).into_response()
            },
            Self::HashError(e) => {
                ResponseBuilder::error("", Some(e.to_string().as_str()), Some("Hash Error"), Some(500)).into_response()
            },
            Self::Serilaztion => {
                ResponseBuilder::error("", Some("Serilaztion Error"), Some("Serilaztion Error"), Some(500)).into_response()
            }
        }
    }
}

