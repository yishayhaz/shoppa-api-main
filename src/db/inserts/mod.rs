mod categories;
mod contact_us;
mod news_letter;
mod prelude;
mod products;
mod sites_visite;
mod stores;
mod users;
mod variants;

pub use categories::*;
pub use contact_us::*;
pub use news_letter::*;
pub use products::*;
pub use sites_visite::*;
pub use stores::*;
pub use users::*;
pub use variants::*;

use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use mongodb::error::{ErrorKind, WriteFailure};

pub enum InsertDocumentErrors {
    UnknownError,
    AlredyExists,
    BsonConversionError,
    PopulatedRefField,
    InvalidArgumentsForModel,
}

impl IntoResponse for InsertDocumentErrors {
    fn into_response(self) -> Response {
        match self {
            Self::UnknownError => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Unknown error while saving document"),
                None,
            )
            .into_response(),
            Self::AlredyExists => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Document alredy exists"),
                Some(409),
            )
            .into_response(),
            Self::BsonConversionError => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Faild parsing a document to bson"),
                Some(500),
            )
            .into_response(),
            Self::PopulatedRefField => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Passed populated field where there should be none"),
                Some(500),
            )
            .into_response(),
            Self::InvalidArgumentsForModel => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Passed invalid arguments to model"),
                Some(500),
            )
            .into_response(),
        }
    }
}

pub fn extract_insert_document_error(error: ErrorKind) -> InsertDocumentErrors {
    match error {
        ErrorKind::Write(e) => match e {
            WriteFailure::WriteConcernError(_) => {}
            WriteFailure::WriteError(we) => {
                if we.code == 11000 {
                    return InsertDocumentErrors::AlredyExists;
                }
            }
            _ => {}
        },
        _ => {}
    };

    InsertDocumentErrors::UnknownError
}
