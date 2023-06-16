use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use shoppa_core::db::{
    aggregations,
    models::{Category, VariantType, VariantValue, Variants},
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
    async fn get_variant_for_extarnel(&self, id: &ObjectId) -> Result<Option<Document>>;
    async fn update_variant_basic(
        &self,
        variant_id: &ObjectId,
        name: &Option<String>,
        type_: &Option<VariantType>,
        new_values: &Option<Vec<VariantValue>>,
    ) -> Result<Option<Variants>>;
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
                Some(vec![
                    aggregations::limit(3),
                    aggregations::project(
                        aggregations::ProjectIdOptions::Keep,
                        vec![Category::fields().name],
                        None,
                    ),
                ]),
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

    async fn get_variant_for_extarnel(&self, id: &ObjectId) -> Result<Option<Document>> {
        let pipeline = [
            aggregations::match_query(&doc! {
                Variants::fields().id: id
            }),
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

        let mut variant = self.aggregate_variants(pipeline, None, None).await?;
        // we can use pop because we are sure that we have only one variant
        // since we are using id as a filter
        Ok(variant.pop())
    }

    async fn update_variant_basic(
        &self,
        variant_id: &ObjectId,
        name: &Option<String>,
        type_: &Option<VariantType>,
        new_values: &Option<Vec<VariantValue>>,
    ) -> Result<Option<Variants>> {
        let mut push = doc! {};

        let mut set = doc! {};

        if !new_values.is_some() & !new_values.as_ref().unwrap().is_empty() {
            push.insert(
                Variants::fields().values,
                doc! {
                    "$each": new_values
                },
            );
        }

        if let Some(name) = name {
            set.insert(Variants::fields().name, name);
        }

        if let Some(type_) = type_ {
            set.insert(Variants::fields().type_, type_);
        }

        let mut update = doc! {};

        if set.is_empty() & push.is_empty() {
            return Err(Error::ApiErrorWithCode(
                "Please provide values to update",
                400,
            ));
        }

        if !set.is_empty() {
            update.insert("$set", set);
        }

        if !push.is_empty() {
            update.insert("$push", push);
        }

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        self.find_and_update_variant_by_id(variant_id, update, Some(options), None)
            .await
    }
}
