use crate::prelude::types::*;
use shoppa_core::db::models::OrderPartStatus;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct UpdateOrderStatusPayload {
    pub status: OrderPartStatus,
}
