use crate::prelude::types::*;
use shoppa_core::db::models::InvoiceType;

#[derive(Debug, Deserialize, Clone)]
pub struct InvoicesQuery {
    pub from: Option<chrono::NaiveDate>,
    pub to: Option<chrono::NaiveDate>,
    pub invoice_type: Option<InvoiceType>,
}
