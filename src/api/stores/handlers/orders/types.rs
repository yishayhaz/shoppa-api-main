use crate::prelude::types::*;
use shoppa_core::db::models::{InvoiceType, OrderPartStatus};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct UpdateOrderStatusPayload {
    pub status: OrderPartStatus,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrdersQuery {
    pub from: Option<chrono::NaiveDate>,
    pub to: Option<chrono::NaiveDate>,
    pub status: Option<OrderPartStatus>,
    pub utm: Option<String>,
}
