use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::{options::AggregateOptions, results::UpdateResult};
use shoppa_core::{
    db::{
        aggregations,
        models::{DBModel, Order, OrderPartStatus, OrderTransaction},
        populate::{FieldPopulate, OrderPopulate, PopulateOptions, ProductsPopulate},
        // populate::OrderPopulate,
        DBConection,
        Pagination,
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
    async fn get_orders_for_store(
        &self,
        pagination: Option<Pagination>,
        store_id: ObjectId,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
        status: Option<OrderPartStatus>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
    async fn get_order_by_id_for_store(
        &self,
        store_id: ObjectId,
        order_id: ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>>;
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

    async fn get_orders_for_store(
        &self,
        pagination: Option<Pagination>,
        store_id: ObjectId,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
        status: Option<OrderPartStatus>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let filter = aggregations::match_query(&doc! {
            Order::fields().parts(true).store: store_id,
        });

        let pipeline = [
            filter.clone(),
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::unwind(Order::fields().parts, false),
            aggregations::match_query(&doc! {
                Order::fields().parts(true).store: store_id,
            }),
            aggregations::project(
                aggregations::ProjectIdOptions::Keep,
                [
                    Order::fields().created_at,
                    Order::fields().updated_at,
                    Order::fields().refunds,
                    Order::fields().user,
                    Order::fields().address,
                    Order::fields().info,
                ],
                Some(doc! {
                    Order::fields().parts: "$parts",
                    Order::fields().parts(false).status: "$parts.status",
                    Order::fields().parts(false).total: "$parts.total",
                    Order::fields().parts(false).total_after_refunds: "$parts.total_after_refunds",
                    Order::fields().parts(false).items: "$parts.items",
                    // notes, utm
                }),
            ),
        ];

        let count = self
            .count_orders_with_aggregation(
                [filter.clone(), aggregations::count("count")],
                options,
                None,
            )
            .await
            .unwrap_or_default();

        let orders = self
            .aggregate_orders(pipeline, None, None)
            .await
            .unwrap_or_default();

        Ok((orders, count))
    }

    async fn get_order_by_id_for_store(
        &self,
        store_id: ObjectId,
        order_id: ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>> {
        let populate_pipeline = OrderPopulate {
            stores: FieldPopulate::None,
            products: FieldPopulate::Nested(ProductsPopulate {
                store: false,
                categories: FieldPopulate::None,
                variants: true,
                options: None,
            }),
            user: FieldPopulate::None,
            options: None,
        }
        .build_pipeline();

        let mut pipeline = vec![aggregations::match_query(&doc! {
            Order::fields().id: order_id,
            Order::fields().parts(true).store: store_id,
        })];

        pipeline.extend(populate_pipeline);

        pipeline.extend([
            aggregations::unwind(Order::fields().parts, false),
            aggregations::match_query(&doc! {
                Order::fields().parts(true).store: store_id,
            }),
            aggregations::project(
                aggregations::ProjectIdOptions::Keep,
                [
                    Order::fields().created_at,
                    Order::fields().updated_at,
                    Order::fields().refunds,
                    Order::fields().user,
                    Order::fields().address,
                    Order::fields().info,
                ],
                Some(doc! {
                    Order::fields().parts: "$parts",
                    Order::fields().parts(false).status: "$parts.status",
                    Order::fields().parts(false).total: "$parts.total",
                    Order::fields().parts(false).total_after_refunds: "$parts.total_after_refunds",
                    Order::fields().parts(false).items: "$parts.items",
                    // notes, utm
                }),
            ),
        ]);

        Ok(self.aggregate_orders(pipeline, options, None).await?.pop())
    }
}
