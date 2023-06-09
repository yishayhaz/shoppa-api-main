mod stores;
mod products;
mod categories;


pub use products::*;
pub use stores::*;
pub use categories::*;

use axum::extract::Extension;
use shoppa_core::db::DBConection;
use std::sync::Arc;
pub type AxumDBExtansion = Extension<Arc<DBConection>>;




