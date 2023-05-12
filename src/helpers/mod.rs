pub mod cookies;
pub mod env;
pub mod extractors;
pub mod parser;
pub mod random;
pub mod security;
pub mod setup;
pub mod types;
pub mod validators;

pub const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 5; // 5MB
