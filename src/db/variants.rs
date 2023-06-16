use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use shoppa_core::db::{
    aggregations,
    models::{Category, Variants},
    DBConection, Pagination,
};

#[async_trait]
pub trait AdminVariantsFunctions {
    async fn validate_variants_exist(&self, ids: &Vec<ObjectId>) -> Result<bool>;
    async fn get_variants_by_ids(&self, ids: &Vec<ObjectId>) -> Result<Vec<Variants>>;
    async fn check_if_variant_is_in_use(&self, id: &ObjectId) -> Result<bool>;
    async fn get_variants_for_extarnel(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<(Vec<Document>, u64)>;
}

#[async_trait]
impl AdminVariantsFunctions for DBConection {
    async fn validate_variants_exist(&self, ids: &Vec<ObjectId>) -> Result<bool> {
        if ids.is_empty() {
            return Ok(true);
        }

        let count = self
            .count_variants(
                Some(doc! {
                    Variants::fields().id: {
                        "$in": ids
                    }
                }),
                None,
                None,
            )
            .await?;

        Ok(count == ids.len() as u64)
    }

    async fn get_variants_by_ids(&self, ids: &Vec<ObjectId>) -> Result<Vec<Variants>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        self.get_variants(
            doc! {
                Variants::fields().id: {
                    "$in": ids
                }
            },
            None,
            None,
            None,
        )
        .await
    }

    async fn check_if_variant_is_in_use(&self, id: &ObjectId) -> Result<bool> {
        let filters = doc! {
            Category::fields().allowed_variants: {
                "$in": [id]
            }
        };

        let count = self.count_categories(Some(filters), None, None).await?;

        Ok(count > 0)
    }

    async fn get_variants_for_extarnel(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let pipeline = [
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                vec![Variants::fields().name, Variants::fields().values, "type"],
                None,
            ),
        ];

        let cursor = db
            .variants
            .aggregate(pipeline, None)
            .await
            .map_err(|e| Error::DBError(("variants", e)))?;

        let variants = cursor.consume().await?;

        let mut count = variants.len() as i64;

        if count < pagination.amount {
            count += pagination.offset;

            return Ok((variants, count as u64));
        }

        let count = db
            .variants
            .count_documents(doc! {}, None)
            .await
            .map_err(|e| Error::DBError(("variants", e)))?;

        Ok((variants, count))
    }
}
