use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::AggregateOptions;
use shoppa_core::db::{
    aggregations,
    models::{Invoice, InvoiceType},
    DBConection, Pagination,
};

#[async_trait]
pub trait InvoicesFunctions {
    async fn get_invoices_for_external(
        &self,
        pagination: Option<Pagination>,
        store_id: Option<ObjectId>,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
        invoice_type: Option<InvoiceType>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
}

#[async_trait]
impl InvoicesFunctions for DBConection {
    async fn get_invoices_for_external(
        &self,
        pagination: Option<Pagination>,
        store_id: Option<ObjectId>,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
        invoice_type: Option<InvoiceType>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        // omer todo:
        // 1. make use of `from`, `to` and `invoice_type` params
        // 2. exclude `original` field

        let pagination = pagination.unwrap_or_default();

        let filter = match store_id {
            Some(store_id) => aggregations::match_query(&doc! {
                Invoice::fields().store: store_id
            }),
            None => aggregations::match_all(),
        };

        // let project_stage = aggregations::project(
        //     aggregations::ProjectIdOptions::Keep,
        //     vec![Variants::fields().name],
        //     None,
        // );

        let pipeline = [
            filter.clone(),
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            // project_stage,
        ];

        let count = self
            .count_invoices_with_aggregation(
                [filter.clone(), aggregations::count("count")],
                options,
                None,
            )
            .await
            .unwrap_or_default();

        let invoices = self
            .aggregate_invoices(pipeline, None, None)
            .await
            .unwrap_or_default();

        Ok((invoices, count))
    }
}
