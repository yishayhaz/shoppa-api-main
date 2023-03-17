mod responses;
pub use responses::*;

use axum::Extension;
use crate::db::DBCollections;
use std::sync::Arc;

pub type DBExtension = Extension<Arc<DBCollections>>;


pub enum Cookeys {
    AccessToken,
    CsrfToken,
    VisitIndicator
}

pub enum HeadKeys {
    CsrfToken,
}

impl Cookeys {
    pub fn get(&self) -> &str {
        match self {
            Self::AccessToken => "bribed_pigeon_here",
            Self::CsrfToken => "a_delicious_pigeon",
            Self::VisitIndicator => "familiar_pigeon"
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