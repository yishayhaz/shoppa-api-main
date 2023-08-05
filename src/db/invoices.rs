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
        store_id: ObjectId,
        from: Option<chrono::NaiveDate>,
        to: Option<chrono::NaiveDate>,
        invoice_type: Option<InvoiceType>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
}

#[async_trait]
impl InvoicesFunctions for DBConection {
    async fn get_invoices_for_external(
        &self,
        pagination: Option<Pagination>,
        store_id: ObjectId,
        from: Option<chrono::NaiveDate>,
        to: Option<chrono::NaiveDate>,
        invoice_type: Option<InvoiceType>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let mut filters = doc! {
            Invoice::fields().store: store_id,
        };

        if from.is_some() || to.is_some() {
            let mut d = doc! {};

            if let Some(from) = from {
                let from =
                    chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", from)).unwrap();
                d.insert("$gte", from);
            }

            if let Some(to) = to {
                let to =
                    chrono::DateTime::parse_from_rfc3339(&format!("{}T23:59:59Z", to)).unwrap();
                d.insert("$lte", to);
            }

            filters.insert(Invoice::fields().created_at, d);
        }

        if let Some(invoice_type) = invoice_type {
            filters.insert(Invoice::fields().type_, invoice_type.to_string());
        }

        let pipeline = [
            aggregations::match_query(&filters),
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::unset(vec![Invoice::fields().original]),
        ];

        let count = self
            .count_invoices_with_aggregation(
                [
                    aggregations::match_query(&filters),
                    aggregations::count("count"),
                ],
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
