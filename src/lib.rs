pub mod api;
pub mod db;
pub mod helpers;
pub mod services;
pub mod error;
pub mod prelude;
#[macro_use]
extern crate lazy_static;


use shoppa_core::db::DBConection;


pub struct AppState {
    pub db: DBConection,
    // file_storage: file_storage::FileStorageClient,
}