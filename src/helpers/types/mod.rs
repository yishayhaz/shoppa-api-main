pub mod error_code;
mod responses;
pub use responses::*;
use crate::services::file_storage::StorageClient;
use strum_macros::{EnumString, Display};

use crate::db::DBCollections;
use axum::Extension;
use std::sync::Arc;

pub type DBExtension = Extension<Arc<DBCollections>>;
pub type StorgeClientExtension = Extension<Arc<StorageClient>>;

#[derive(EnumString, Display)]
pub enum Cookeys {
    #[strum(to_string="bribed_pigeon_here")]
    AccessToken,
    #[strum(to_string="a_delicious_pigeon")]
    CsrfToken,
    #[strum(to_string="familiar_pigeon")]
    VisitIndicator,
    #[strum(to_string="lab_pigeon")]
    DebugingCookie
}

#[derive(EnumString)]
pub enum HeadKeys {
    #[strum(to_string="x-top_secret_pigeon")]
    CsrfToken,
}

pub enum MyOption<T> {
    None,
    Some(T),
}

pub const MAX_COOKIE_EXP: f64 = (u64::pow(2, 31) - 1) as f64;


impl<T> Into<Option<T>> for MyOption<T> {
    fn into(self) -> Option<T> {
        match self {
            Self::None => Option::None,
            Self::Some(v) => Option::Some(v)
        }
    }
}