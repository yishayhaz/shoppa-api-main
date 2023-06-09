pub mod error_code;
mod responses;
pub use responses::*;
use shoppa_core::file_storage::StorageClient;
use strum_macros::{Display, EnumString};

use axum::Extension;
use std::sync::Arc;

pub type AxumStorgeClientExtension = Extension<Arc<StorageClient>>;

#[derive(EnumString, Display)]
pub enum Cookeys {
    #[strum(to_string = "bribed_pigeon_here")]
    AccessToken,
    #[strum(to_string = "a_delicious_pigeon")]
    CsrfToken,
    #[strum(to_string = "familiar_pigeon")]
    VisitIndicator,
    #[strum(to_string = "lab_pigeon")]
    DebugingCookie,
}

#[derive(EnumString)]
pub enum HeadKeys {
    #[strum(to_string = "x-top_secret_pigeon")]
    CsrfToken,
}
