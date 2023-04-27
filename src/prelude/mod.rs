use axum::response::Response;
pub mod handlers;
pub mod types;
pub mod db_models;


pub use crate::error::Error;


pub type Result<T> = core::result::Result<T, Error>;
pub type StdResult<T, E> = core::result::Result<T, E>;
pub type HandlerResult = Result<Response>;