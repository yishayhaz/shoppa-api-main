mod contact_us;
mod news_letter;
mod sites_visite;

pub use contact_us::*;
pub use news_letter::*;
pub use sites_visite::*;

use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
pub enum InsertDocumentErrors {
    UnknownError,
    AlredyExists,
}

impl IntoResponse for InsertDocumentErrors {
    fn into_response(self) -> Response {
        match self {
            Self::UnknownError => ResponseBuilder::<u16>::error(
                None,
                Some(String::from("Unknown error while saving document")),
                None,
            )
            .into_response(),
            Self::AlredyExists => ResponseBuilder::<u16>::error(
                None,
                Some(String::from("Document alredy exists")),
                Some(409),
            )
            .into_response(),
        }
    }
}
