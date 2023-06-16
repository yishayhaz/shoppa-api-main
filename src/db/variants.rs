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
            aggregations::lookup::<Category>(
                Variants::fields().id,
                Category::fields().allowed_variants,
                "categories",
                Some(vec![aggregations::project(
                    aggregations::ProjectIdOptions::Keep,
                    vec![Category::fields().name],
                    None,
                )]),
                None,
            ),
            aggregations::project(
                aggregations::ProjectIdOptions::Keep,
                vec![
                    Variants::fields().name,
                    Variants::fields().values,
                    "type",
                    "categories",
                ],
                Some(doc! {
                    "deletable": {
                        "$cond": {
                            "if": {
                                "$eq": [
                                    // No need to use safe array access because we are using $lookup
                                    {
                                        "$size": "$categories"
                                    },
                                    0
                                ]
                            },
                            "then": true,
                            "else": false
                        }
                    }
                }),
            ),
        ];

        let variants = self.aggregate_variants(pipeline, None, None).await?;

        let count = variants.len();

        if !pagination.need_count(count) {
            return Ok((variants, pagination.calculate_total(count)));
        }

        let count = self.count_variants(None, None, None).await?;

        Ok((variants, count))
    }
}
