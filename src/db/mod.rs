mod categories;
mod products;
mod stores;
mod users;
mod variants;
mod store_users;
mod checkout_session;

pub use categories::*;
pub use products::*;
pub use stores::*;
pub use users::*;
pub use variants::*;
pub use store_users::*;
pub use checkout_session::*;

use axum::extract::Extension;
use shoppa_core::db::DBConection;
use std::sync::Arc;
pub type AxumDBExtansion = Extension<Arc<DBConection>>;
