use axum::response::Response;
pub mod types;
pub use axum::async_trait;
pub use shoppa_core::prelude::*;

pub type HandlerResult = Result<Response>;
pub use crate::helpers::env::ENV_VARS;
