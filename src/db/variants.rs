use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use shoppa_core::db::{
    aggregations,
    models::{Category, VariantType, VariantValue, Variants, Product},
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
    async fn update_variant_value(
        &self,
        variant_id: &ObjectId,
        value_id: &ObjectId,
        value: Option<String>,
        label: Option<String>,
    ) -> Result<Option<Variants>>;
    async fn delete_variant_value(
        &self,
        variant_id: &ObjectId,
        value_id: &ObjectId,
    ) -> Result<Option<Variants>>;
    async fn check_if_variant_value_is_in_use(
        &self,
        variant_id: &ObjectId,
        value_id: &ObjectId,
    ) -> Result<bool>;
    async fn get_variants_by_categories(
        &self,
        pagination: Option<Pagination>,
        categories_ids: Vec<ObjectId>,
        free_text: Option<String>,
    ) -> Result<Vec<Document>>;
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

        if count > 0 {
            return Ok(true);
        }

        let product_filter = doc! {
            Product::fields().variants: {
                "$in": [id]
            }
        };

        let count = self.count_products(Some(product_filter), None, None).await?;

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
            aggregations::lookup::<Product>(
                Variants::fields().id,
                Product::fields().variants,
                "products",
                Some(vec![
                    aggregations::limit(3),
                    aggregations::project(
                        aggregations::ProjectIdOptions::Keep,
                        vec![Product::fields().name],
                        None,
                    ),
                ]),
                None
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
                                "$and": [
                                    {
                                        "$eq": [
                                            {
                                                "$size": "$categories"
                                            },
                                            0
                                        ]
                                    },
                                    {
                                        "$eq": [
                                            {
                                                "$size": "$products"
                                            },
                                            0
                                        ]
                                    }
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
            aggregations::lookup::<Product>(
                Variants::fields().id,
                Product::fields().variants,
                "products",
                Some(vec![
                    aggregations::limit(3),
                    aggregations::project(
                        aggregations::ProjectIdOptions::Keep,
                        vec![Product::fields().name],
                        None,
                    ),
                ]),
                None
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
                                "$and": [
                                    {
                                        "$eq": [
                                            {
                                                "$size": "$categories"
                                            },
                                            0
                                        ]
                                    },
                                    {
                                        "$eq": [
                                            {
                                                "$size": "$products"
                                            },
                                            0
                                        ]
                                    }
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

        if new_values.is_some() & !new_values.as_ref().unwrap().is_empty() {
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

        let mut update = doc! {
            "$currentDate": {
                Variants::fields().updated_at: true
            }
        };

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

    async fn update_variant_value(
        &self,
        variant_id: &ObjectId,
        value_id: &ObjectId,
        value: Option<String>,
        label: Option<String>,
    ) -> Result<Option<Variants>> {
        let filters = doc! {
            Variants::fields().id: variant_id,
            Variants::fields().values(true).id: value_id
        };

        let mut set = doc! {};

        if let Some(value) = value {
            set.insert(
                format!(
                    "{}.$.{}",
                    Variants::fields().values,
                    Variants::fields().values(false).value
                ),
                value,
            );
        }

        if let Some(label) = label {
            set.insert(
                format!(
                    "{}.$.{}",
                    Variants::fields().values,
                    Variants::fields().values(false).label
                ),
                label,
            );
        }

        let update = doc! {
            "$set": set,
            "$currentDate": {
                format!("{}.$.{}", 
                Variants::fields().values, 
                Variants::fields().values(false).updated_at): true
            }
        };

        self.find_and_update_variant(filters, update, None, None)
            .await
    }

    async fn delete_variant_value(
        &self,
        variant_id: &ObjectId,
        value_id: &ObjectId,
    ) -> Result<Option<Variants>> {

        let filters = doc! {
            Variants::fields().id: variant_id,
            Variants::fields().values(true).id: value_id
        };

        let update = doc! {
            "$pull": {
                Variants::fields().values: {
                    Variants::fields().values(false).id: value_id
                }
            },
            "$currentDate": {
                Variants::fields().updated_at: true
            }
        };

        self.find_and_update_variant(filters, update, None, None)
            .await
    }

    async fn check_if_variant_value_is_in_use(
        &self,
        variant_id: &ObjectId,
        value_id: &ObjectId,
    ) -> Result<bool> {
        

        // TODO


        Ok(true)
    }

    async fn get_variants_by_categories(
        &self,
        pagination: Option<Pagination>,
        categories_ids: Vec<ObjectId>,
        free_text: Option<String>,
    ) -> Result<Vec<Document>> {

        let pagination = pagination.unwrap_or_default();

        let filters = match free_text {
            Some(free_text) => aggregations::match_query(&doc! {
                Variants::fields().name: {
                    "$regex": free_text,
                    "$options": "i"
                }
            }),
            None => aggregations::match_query(&doc! {}),
        };

        let pipeline = match categories_ids.is_empty() {
            true => [
                filters,
                aggregations::limit(15)
            ],
            false => [
                filters,
                aggregations::limit(15)
            ]
        };

        self.aggregate_variants(pipeline, None, None).await

    }
}
