pub mod error_code;
mod responses;
pub use responses::*;

use crate::db::DBCollections;
use axum::Extension;
use std::sync::Arc;

pub type DBExtension = Extension<Arc<DBCollections>>;

pub enum Cookeys {
    AccessToken,
    CsrfToken,
    VisitIndicator,
    DebugingCookie
}

pub enum HeadKeys {
    CsrfToken,
}

pub enum MyOption<T> {
    None,
    Some(T),
}

pub const MAX_COOKIE_EXP: f64 = (u64::pow(2, 31) - 1) as f64;

impl Cookeys {
    pub fn get(&self) -> &str {
        match self {
            Self::AccessToken => "bribed_pigeon_here",
            Self::CsrfToken => "a_delicious_pigeon",
            Self::VisitIndicator => "familiar_pigeon",
            Self::DebugingCookie => "lab_pigeon"
        }
    }
}

impl HeadKeys {
    pub fn get(&self) -> &str {
        match self {
            Self::CsrfToken => "x-top_secret_pigeon",
        }
    }
}

impl<T> MyOption<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::None => Option::None,
            Self::Some(v) => Option::Some(v)
        }
    }
}