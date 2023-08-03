mod categories;
mod checkout_session;
mod invoices;
mod orders;
mod products;
mod store_users;
mod stores;
mod users;
mod variants;

pub use categories::*;
pub use checkout_session::*;
pub use invoices::*;
pub use orders::*;
pub use products::*;
pub use store_users::*;
pub use stores::*;
pub use users::*;
pub use variants::*;

use axum::extract::Extension;
use shoppa_core::db::DBConection;
use std::sync::Arc;
pub type AxumDBExtansion = Extension<Arc<DBConection>>;
