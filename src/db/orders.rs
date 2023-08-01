use crate::prelude::*;
use axum::async_trait;
use bson::doc;
use mongodb::results::UpdateResult;
use shoppa_core::{
    db::{
        models::{DBModel, Order, OrderTransaction},
        // populate::OrderPopulate,
        DBConection,
    },
    payments::types::TransactionInfo,
};

#[async_trait]
pub trait OrderFunctions {
    async fn update_order_after_payment(
        &self,
        order: &Order,
        transaction_info: TransactionInfo,
        card_holder_name: String,
    ) -> Result<UpdateResult>;
}

#[async_trait]
impl OrderFunctions for DBConection {
    async fn update_order_after_payment(
        &self,
        order: &Order,
        transaction_info: TransactionInfo,
        card_holder_name: String,
    ) -> Result<UpdateResult> {
        let order_transaction = OrderTransaction {
            token: transaction_info.token,
            cc_last4: transaction_info.cc_last4,
            cc_length: transaction_info.cc_length,
            cc_company: transaction_info.cc_company,
            holder_name: card_holder_name,
        };

        let update = doc! {
            "$set": {
                Order::fields().transaction: order_transaction,
            }
        };

        self.update_order_by_id(order.id().unwrap(), update, None, None)
            .await
    }
}
