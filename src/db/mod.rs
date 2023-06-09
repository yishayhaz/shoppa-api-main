mod categories;
mod products;
mod stores;
mod users;

pub use categories::*;
pub use products::*;
pub use stores::*;
pub use users::*;

use axum::extract::Extension;
use shoppa_core::db::DBConection;
use std::sync::Arc;
pub type AxumDBExtansion = Extension<Arc<DBConection>>;
