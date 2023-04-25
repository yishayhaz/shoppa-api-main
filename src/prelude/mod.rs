use axum::response::Response;
pub mod handlers;
pub mod types;


pub use crate::error::Error;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

pub type HandlerResult = core::result::Result<Response, Error>;