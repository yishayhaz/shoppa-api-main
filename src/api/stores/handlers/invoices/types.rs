use crate::prelude::types::*;
use shoppa_core::db::models::InvoiceType;

#[derive(Debug, Deserialize, Clone)]
pub struct InvoicesQuery {
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
    pub invoice_type: Option<InvoiceType>,
}
