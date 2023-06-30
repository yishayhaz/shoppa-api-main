use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Bson, Document};
use chrono::Utc;
use mongodb::{
    options::{AggregateOptions, FindOneAndUpdateOptions, UpdateOptions},
    results::UpdateResult,
};
use serde::Deserialize;
use shoppa_core::{
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{
            EmbeddedDocument, FileDocument, ProducdBrandField, Product, ProductItemStatus,
            ProductStatus, Variants,
        },
        DBConection, Pagination, Sorter,
    },
    parser::FieldPatch,
};
use strum_macros::EnumString;

#[derive(Deserialize, Debug, Clone, PartialEq, EnumString)]
pub enum ProductSortBy {
    #[serde(alias = "popularity", alias = "pop", alias = "p", alias = "Popularity")]
    Popularity,
    #[serde(alias = "date", alias = "da", alias = "d", alias = "Date")]
    Date,
    #[serde(alias = "relevance", alias = "rel", alias = "r", alias = "Relevance")]
    Relevance,
}

impl Default for ProductSortBy {
    fn default() -> Self {
        Self::Relevance
    }
}

#[async_trait]
pub trait ProductFunctions {
    async fn add_view_to_product(
        &self,
        product_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;
    async fn autocomplete_products_search(
        &self,
        free_text: String,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;
    async fn get_one_product_for_extarnel(
        &self,
        product_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>>;
    async fn get_products_for_extarnel(
        &self,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        free_text: Option<String>,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
    async fn random_autocomplete_products_search(
        &self,
        amount: Option<u8>,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;
}

#[async_trait]
pub trait AdminProductFunctions {
    async fn add_asset_to_product(
        &self,
        product_id: &ObjectId,
        asset: &FileDocument,
        items_ids: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn edit_product_item(
        &self,
        product_id: &ObjectId,
        item_id: &ObjectId,
        price: Option<f64>,
        in_storage: Option<u64>,
        name: FieldPatch<String>,
        images_refs: Option<Vec<ObjectId>>,
        sku: FieldPatch<String>,
        info: FieldPatch<String>,
        status: Option<ProductItemStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn edit_product_by_id(
        &self,
        product_id: &ObjectId,
        name: Option<String>,
        keywords: Option<Vec<String>>,
        brand: Option<String>,
        description: Option<String>,
        feature_bullet_points: Option<Vec<String>>,
        warranty: Option<f32>,
        status: Option<ProductStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn delete_product_file(
        &self,
        product_id: &ObjectId,
        file_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn get_products_for_admins(
        &self,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        free_text: Option<String>,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        status: Option<ProductStatus>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;

    async fn delete_product_item(
        &self,
        product_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;
}

#[async_trait]
pub trait StoreProductFunctions {
    async fn add_asset_to_product(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        asset: &FileDocument,
        items_ids: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn edit_product_item(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        item_id: &ObjectId,
        price: Option<f64>,
        in_storage: Option<u64>,
        name: FieldPatch<String>,
        images_refs: Option<Vec<ObjectId>>,
        sku: FieldPatch<String>,
        info: FieldPatch<String>,
        status: Option<ProductItemStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn edit_product_by_id(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        name: Option<String>,
        keywords: Option<Vec<String>>,
        brand: Option<String>,
        description: Option<String>,
        feature_bullet_points: Option<Vec<String>>,
        warranty: Option<f32>,
        status: Option<ProductStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn delete_product_file(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        file_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn get_products_for_store_manager(
        &self,
        store_id: &ObjectId,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        free_text: Option<String>,
        category_id: Option<ObjectId>,
        status: Option<ProductStatus>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;

    async fn delete_product_item(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;
}

#[async_trait]
impl ProductFunctions for DBConection {
    async fn add_view_to_product(
        &self,
        product_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let update = doc! {
            "$inc": {
                Product::fields().analytics(true).views: 1
            }
        };

        self.find_and_update_product_by_id(product_id, update, options, None)
            .await
    }

    async fn autocomplete_products_search(
        &self,
        free_text: String,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let mut filters = vec![
            doc! {
                "text": {
                    "path": Product::fields().status,
                    "query": ProductStatus::Active
                }

            },
            doc! {
                "text": {
                    "path": Product::fields().items(true).status,
                    "query": ProductItemStatus::Active
                }
            },
        ];

        if let Some(store_id) = store_id {
            filters.push(doc! {
                "equals": {
                    "value": store_id,
                    "path": Product::fields().store(true).id
                }
            });
        };

        if let Some(category_id) = category_id {
            filters.push(doc! {
                "equals": {
                    "value": category_id,
                    "path": Product::fields().categories(true).ids
                }
            });
        }

        // TODO in the future we need to use the embeddeddocuments search to return the must
        // relevant product item and not the first one
        let pipeline = [
            aggregations::autocomplete_products_search(&free_text, filters),
            aggregations::add_score_meta(),
            aggregations::sort_by_score(),
            aggregations::limit(10),
            aggregations::project(
                ProjectIdOptions::Keep,
                [Product::fields().name],
                Some(doc! {
                    "item_id": {
                        "$first":
                        format!("${}", Product::fields().items(true).id)
                    },
                    "views": format!("${}", Product::fields().analytics(true).views),
                }),
            ),
        ];

        self.aggregate_products(pipeline, options, None).await
    }

    async fn get_one_product_for_extarnel(
        &self,
        product_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>> {
        let filter = vec![
            doc! {
                "equals": {
                    "path": Product::fields().id,
                    "value": product_id,
                }
            },
            doc! {
                "text": {
                    "path": Product::fields().status,
                    "query": ProductStatus::Active
                }

            },
            doc! {
                "text": {
                    "path": Product::fields().items(true).status,
                    "query": ProductItemStatus::Active
                }
            },
        ];

        let pipeline = [
            aggregations::search(doc! {
                "compound": {
                    "filter": filter
                }
            }),
            aggregations::lookup_product_variants(Some(vec![aggregations::project(
                ProjectIdOptions::Keep,
                [
                    Variants::fields().type_,
                    Variants::fields().name,
                    Variants::fields().values(true).label,
                    Variants::fields().values(true).value,
                    Variants::fields().values(true).id,
                ],
                None,
            )])),
            aggregations::add_fields(doc! {
                // only items that are active or sold out
                Product::fields().items: {
                    "$filter": {
                        "input": format!("${}", Product::fields().items),
                        "as": "item",
                        "cond": {
                            "$eq": [
                                format!("$$item.{}", Product::fields().items(false).status),
                                ProductItemStatus::Active
                            ]
                        }
                    }
                },
                // only assets that are public and not hidden
                Product::fields().assets: {
                    "$filter": {
                        "input": format!("${}", Product::fields().assets),
                        "as": "asset",
                        "cond": {
                            "$and": [
                                {
                                    "$eq": [
                                        format!("$$asset.{}", Product::fields().assets(false).hidden),
                                        false
                                    ]
                                },
                                {
                                    "$eq": [
                                        format!("$$asset.{}", Product::fields().assets(false).public),
                                        true
                                    ]
                                }
                            ]
                        }
                    }
                },
            }),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    Product::fields().created_at,
                    Product::fields().brand,
                    Product::fields().name,
                    Product::fields().description,
                    Product::fields().keywords,
                    Product::fields().store,
                    Product::fields().categories,
                    Product::fields().variants,
                    Product::fields().analytics(true).views,
                    // Product items fields to return
                    Product::fields().items(true).id,
                    Product::fields().items(true).price,
                    Product::fields().items(true).in_storage,
                    Product::fields().items(true).variants,
                    Product::fields().items(true).name,
                    Product::fields().items(true).assets_refs,
                    Product::fields().items(true).sku,
                    Product::fields().items(true).info,
                    Product::fields().items(true).status,
                    // Product assets fields to return
                    Product::fields().assets(true).id,
                    Product::fields().assets(true).file_name,
                    Product::fields().assets(true).path,
                    Product::fields().assets(true).size,
                    Product::fields().assets(true).mime_type,
                    Product::fields().assets(true).file_type,
                ],
                None,
            ),
        ];

        let products = self.aggregate_products(pipeline, options, None).await?;

        Ok(products.into_iter().next())
    }

    async fn get_products_for_extarnel(
        &self,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        free_text: Option<String>,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();
        let sorting = sorting.unwrap_or_default();

        let sort_stage = match sorting.sort_by {
            ProductSortBy::Date => aggregations::sort(doc! {
                Product::fields().created_at: &sorting.direction
            }),
            ProductSortBy::Popularity => aggregations::sort(doc! {
                Product::fields().analytics(true).views: &sorting.direction
            }),
            ProductSortBy::Relevance => {
                if free_text.is_some() {
                    aggregations::sort(doc! {
                        "score": &sorting.direction
                    })
                } else {
                    aggregations::sort(doc! {
                        Product::fields().created_at: -1
                    })
                }
            }
        };

        let filters = {
            let mut f = vec![
                doc! {
                    "text": {
                        "path": Product::fields().status,
                        "query": ProductStatus::Active
                    }

                },
                doc! {
                    "text": {
                        "path": Product::fields().items(true).status,
                        "query": ProductItemStatus::Active
                    }
                },
            ];

            if let Some(store_id) = store_id {
                f.push(doc! {
                    "equals": {
                        "value": store_id,
                        "path": Product::fields().store(true).id
                    }
                });
            };

            if let Some(category_id) = category_id {
                f.push(doc! {
                    "equals": {
                        "value": category_id,
                        "path": Product::fields().categories(true).ids
                    }
                });
            }
            f
        };

        let pipeline = [
            aggregations::search_products(&free_text, &filters, Some(1)),
            aggregations::add_score_meta(),
            sort_stage,
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                vec![
                    Product::fields().brand,
                    Product::fields().name,
                    Product::fields().keywords,
                    Product::fields().analytics,
                    Product::fields().categories,
                    Product::fields().created_at,
                    Product::fields().store,
                    Product::fields().assets,
                ],
                // In the future return the most relevant item
                Some(doc! {
                    "item": {
                        "$arrayElemAt": [
                            {
                            "$filter": {
                                "input": format!("${}", Product::fields().items),
                                "as": "item",
                                "cond": {
                                    "$eq": [
                                        format!("$$item.{}", Product::fields().items(false).status),
                                        ProductItemStatus::Active
                                    ]
                                }
                            }
                        },
                            0
                        ]
                    }
                }),
            ),
        ];

        let products = self
            .aggregate_products(pipeline, options.clone(), None)
            .await?;

        let count = products.len();

        if !pagination.need_count(count) {
            return Ok((products, pagination.calculate_total(count)));
        }

        let count = self
            .count_products_with_aggregation(
                [
                    aggregations::search_products(&free_text, &filters, Some(1)),
                    aggregations::count("count"),
                ],
                options,
                None,
            )
            .await?;

        Ok((products, count))
    }

    async fn random_autocomplete_products_search(
        &self,
        amount: Option<u8>,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let amount = amount.unwrap_or(10) as i64;

        let from_pool = amount * 10;

        let filters = {
            let mut f = vec![
                doc! {
                    "text": {
                        "path": Product::fields().status,
                        "query": ProductStatus::Active
                    }

                },
                doc! {
                    "text": {
                        "path": Product::fields().items(true).status,
                        "query": ProductItemStatus::Active
                    }
                },
            ];

            if let Some(store_id) = store_id {
                f.push(doc! {
                    "equals": {
                        "value": store_id,
                        "path": Product::fields().store(true).id
                    }
                });
            };

            if let Some(category_id) = category_id {
                f.push(doc! {
                    "equals": {
                        "value": category_id,
                        "path": Product::fields().categories(true).ids
                    }
                });
            }
            f
        };

        let pipeline = [
            aggregations::search_products(&None, &filters, Some(0)),
            aggregations::sort(doc! {
                Product::fields().analytics(true).views: -1
            }),
            aggregations::limit(from_pool),
            aggregations::sample(amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                [Product::fields().name],
                Some(doc! {
                    "item_id": {
                        "$getField": {
                            "field": "_id",
                            "input": aggregations::random_array_element(&format!("${}", Product::fields().items)),
                        }
                    },
                    "views": format!("${}", Product::fields().analytics(true).views),
                }),
            ),
        ];

        self.aggregate_products(pipeline, options, None).await
    }
}

#[async_trait]
impl AdminProductFunctions for DBConection {
    async fn add_asset_to_product(
        &self,
        product_id: &ObjectId,
        asset: &FileDocument,
        items_ids: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let mut options = options.unwrap_or_default();
        // the update push operation
        let mut push = doc! {
            Product::fields().assets: asset.into_bson()?
        };
        // if items ids are provided, then we need to push the asset to the items refs
        if let Some(items_ids) = items_ids {
            // adding the asset to the items refs using the array filter
            push.insert(
                format!(
                    "{}.$[item].{}",
                    Product::fields().items,
                    Product::fields().items(false).assets_refs
                ),
                asset.into_bson()?,
            );

            let item_filter = doc! {
                "item": {
                    Product::fields().items(true).id: {
                        "$in": items_ids
                    }
                }
            };

            // adding the array filter to the options, if there is already an array filter
            let mut array_filter = options.array_filters.unwrap_or_default();

            array_filter.push(item_filter);

            options.array_filters = Some(array_filter);
        }

        let update = doc! {
            "$push": push
        };

        self.find_and_update_product_by_id(product_id, update, options.into(), None)
            .await
    }

    async fn edit_product_item(
        &self,
        product_id: &ObjectId,
        item_id: &ObjectId,
        price: Option<f64>,
        in_storage: Option<u64>,
        name: FieldPatch<String>,
        assets_refs: Option<Vec<ObjectId>>,
        sku: FieldPatch<String>,
        info: FieldPatch<String>,
        status: Option<ProductItemStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().items(true).id: item_id
        };

        let mut update = doc! {};

        if let Some(price) = price {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).price
            );
            update.insert(field, price);
        }

        if let Some(in_storage) = in_storage {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).in_storage
            );
            update.insert(field, in_storage as i64);
        }

        if FieldPatch::Missing != name {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).name
            );
            update.insert(field, name.into_option());
        }

        if let Some(assets_refs) = assets_refs {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).assets_refs
            );
            update.insert(field, assets_refs);
        }

        if FieldPatch::Missing != sku {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).sku
            );
            update.insert(field, sku.into_option());
        }

        if FieldPatch::Missing != info {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).info
            );
            update.insert(field, info.into_option());
        }

        if let Some(status) = status {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).status
            );
            update.insert(field, status);
        }

        let update_at_field = format!(
            "{}.$.{}",
            Product::fields().items,
            Product::fields().items(false).updated_at
        );

        let update = doc! {
            "$set": update,
            "$currentDate": {
                update_at_field: true
            }
        };

        self.find_and_update_product(filters, update, options, None)
            .await
    }

    async fn edit_product_by_id(
        &self,
        product_id: &ObjectId,
        name: Option<String>,
        keywords: Option<Vec<String>>,
        brand: Option<String>,
        description: Option<String>,
        feature_bullet_points: Option<Vec<String>>,
        warranty: Option<f32>,
        status: Option<ProductStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let mut update = doc! {};

        if let Some(name) = name {
            update.insert(Product::fields().name, name);
        }

        if let Some(keywords) = keywords {
            update.insert(Product::fields().keywords, keywords);
        }

        if let Some(brand) = brand {
            // In the future when the brand is a reference to a brand document
            // this will need to be changed
            update.insert(Product::fields().brand, ProducdBrandField::new(brand));
        }

        if let Some(description) = description {
            update.insert(Product::fields().description, description);
        }

        if let Some(feature_bullet_points) = feature_bullet_points {
            update.insert(
                Product::fields().feature_bullet_points,
                feature_bullet_points,
            );
        }

        if let Some(warranty) = warranty {
            update.insert(Product::fields().warranty, warranty);
        }

        if let Some(status) = status {
            update.insert(Product::fields().status, status);
        }

        let update = doc! {
            "$set": update,
            "$currentDate": {
                Product::fields().updated_at: true
            }
        };

        self.find_and_update_product_by_id(product_id, update, options, None)
            .await
    }

    async fn delete_product_file(
        &self,
        product_id: &ObjectId,
        file_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().assets(true).id: file_id
        };

        let update = doc! {
            "$pull": {
                Product::fields().assets: {
                    Product::fields().assets(false).id: file_id
                },
                Product::fields().items(true).assets_refs: file_id
            }
        };

        self.find_and_update_product(filters, update, options, None)
            .await
    }

    async fn get_products_for_admins(
        &self,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        free_text: Option<String>,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
        status: Option<ProductStatus>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();
        let sorting = sorting.unwrap_or_default();

        let sort_stage = match sorting.sort_by {
            ProductSortBy::Date => aggregations::sort(doc! {
                Product::fields().created_at: &sorting.direction
            }),
            ProductSortBy::Popularity => aggregations::sort(doc! {
                Product::fields().analytics(true).views: &sorting.direction
            }),
            ProductSortBy::Relevance => {
                if free_text.is_some() {
                    aggregations::sort(doc! {
                        "score": &sorting.direction
                    })
                } else {
                    aggregations::sort(doc! {
                        Product::fields().created_at: -1
                    })
                }
            }
        };

        let filters = {
            let mut f = vec![];

            if let Some(store_id) = store_id {
                f.push(doc! {
                    "equals": {
                        "value": store_id,
                        "path": Product::fields().store(true).id
                    }
                });
            };

            if let Some(category_id) = category_id {
                f.push(doc! {
                    "equals": {
                        "value": category_id,
                        "path": Product::fields().categories(true).ids
                    }
                });
            }

            if let Some(status) = status {
                f.push(doc! {
                    "text": {
                        "query": status,
                        "path": Product::fields().status
                    }
                });
            }

            f
        };

        let pipeline = [
            aggregations::search_products(&free_text, &filters, Some(1)),
            aggregations::add_score_meta(),
            sort_stage,
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
        ];

        let products = self
            .aggregate_products(pipeline, options.clone(), None)
            .await?;

        let count = products.len();

        if !pagination.need_count(count) {
            return Ok((products, pagination.calculate_total(count)));
        }

        let count = self
            .count_products_with_aggregation(
                [
                    aggregations::search_products(&free_text, &filters, Some(1)),
                    aggregations::count("count"),
                ],
                options,
                None,
            )
            .await?;

        Ok((products, count))
    }

    async fn delete_product_item(
        &self,
        product_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().items(true).id: item_id
        };

        let update = doc! {
            "$set": {
                format!("{}.$.{}", Product::fields().items, Product::fields().items(false).status): ProductItemStatus::Deleted
            },
            "$currentDate": {
                format!("{}.$.{}", Product::fields().items, Product::fields().items(false).updated_at): true
            }
        };

        self.update_product(filters, update, options, None).await
    }
}

#[async_trait]
impl StoreProductFunctions for DBConection {
    async fn add_asset_to_product(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        asset: &FileDocument,
        items_ids: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let mut options = options.unwrap_or_default();
        // the update push operation
        let mut push = doc! {
            Product::fields().assets: asset.into_bson()?
        };
        // if items ids are provided, then we need to push the asset to the items refs
        if let Some(items_ids) = items_ids {
            // adding the asset to the items refs using the array filter
            push.insert(
                format!(
                    "{}.$[item].{}",
                    Product::fields().items,
                    Product::fields().items(false).assets_refs
                ),
                asset.into_bson()?,
            );

            let item_filter = doc! {
                "item": {
                    Product::fields().items(true).id: {
                        "$in": items_ids
                    }
                }
            };

            // adding the array filter to the options, if there is already an array filter
            let mut array_filter = options.array_filters.unwrap_or_default();

            array_filter.push(item_filter);

            options.array_filters = Some(array_filter);
        }

        let update = vec![
            doc! {
                "$push": push
            },
            doc! {
                "$set": {
                    Product::fields().status: product_status_update()
                }
            },
        ];

        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().store(true).id: store_id,
            Product::fields().status: {
                "$nin": [ProductStatus::Deleted, ProductStatus::Banned]
            }
        };

        self.find_and_update_product(filters, update, options.into(), None)
            .await
    }

    async fn edit_product_item(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        item_id: &ObjectId,
        price: Option<f64>,
        in_storage: Option<u64>,
        name: FieldPatch<String>,
        assets_refs: Option<Vec<ObjectId>>,
        sku: FieldPatch<String>,
        info: FieldPatch<String>,
        status: Option<ProductItemStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().store(true).id: store_id,
            Product::fields().items: {
                "$elemMatch": {
                    Product::fields().items(true).id: item_id,
                    Product::fields().items(true).status: {
                        "$ne": ProductItemStatus::Deleted
                    }
                }
            },
            Product::fields().status: {
                "$nin": [ProductStatus::Deleted, ProductStatus::Banned]
            }
        };

        let mut update_doc = Document::new();

        if let Some(price) = price {
            update_doc.insert(Product::fields().items(false).price, price);
        }

        if let Some(in_storage) = in_storage {
            update_doc.insert(Product::fields().items(false).in_storage, in_storage as i64);
        }

        if FieldPatch::Missing != name {
            update_doc.insert(Product::fields().items(false).name, name.into_option());
        }

        if let Some(assets_refs) = assets_refs {
            update_doc.insert(Product::fields().items(false).assets_refs, assets_refs);
        }

        if FieldPatch::Missing != sku {
            update_doc.insert(Product::fields().items(false).sku, sku.into_option());
        }

        if FieldPatch::Missing != info {
            update_doc.insert(Product::fields().items(false).info, info.into_option());
        }

        if let Some(status) = status {
            update_doc.insert(Product::fields().items(false).status, status);
        }

        if update_doc.is_empty() {
            return Err(Error::Static("No update fields provided"));
        }

        update_doc.insert(Product::fields().items(false).updated_at, Utc::now());

        let update = vec![doc! {
            "$set": {
                Product::fields().status: product_status_update(),
                Product::fields().items: {
                    "$map": {
                        "input": format!("${}", Product::fields().items),
                        "as": "item",
                        "in": {
                            "$cond": {
                                "if": {
                                    "$eq": ["$$item.{}", Product::fields().items(false).id]
                                },
                                "then": {
                                    "$mergeObjects": ["$$item", update_doc]
                                },
                                "else": "$$item"
                            }
                        }
                    }
                }
            }
        }];

        self.find_and_update_product(filters, update, options, None)
            .await
    }

    async fn edit_product_by_id(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        name: Option<String>,
        keywords: Option<Vec<String>>,
        brand: Option<String>,
        description: Option<String>,
        feature_bullet_points: Option<Vec<String>>,
        warranty: Option<f32>,
        status: Option<ProductStatus>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().store(true).id: store_id,
            Product::fields().status: {
                "$nin": [ProductStatus::Deleted, ProductStatus::Banned]
            }
        };

        let mut update_status: bool = false;

        let mut update = doc! {};

        if let Some(name) = name {
            update.insert(Product::fields().name, name);
            update_status = true;
        }

        if let Some(keywords) = keywords {
            update.insert(Product::fields().keywords, keywords);
            update_status = true;
        }

        if let Some(brand) = brand {
            // In the future when the brand is a reference to a brand document
            // this will need to be changed
            update.insert(Product::fields().brand, ProducdBrandField::new(brand));
            update_status = true;
        }

        if let Some(description) = description {
            update.insert(Product::fields().description, description);
            update_status = true;
        }

        if let Some(feature_bullet_points) = feature_bullet_points {
            update.insert(
                Product::fields().feature_bullet_points,
                feature_bullet_points,
            );
            update_status = true;
        }

        if let Some(warranty) = warranty {
            update.insert(Product::fields().warranty, warranty);
            update_status = true;
        }

        // If a update status is needed, there is no need to update the status
        if let Some(status) = status {
            // If a status update is needed the status supplied by the user
            // can only be to pending or draft
            if update_status
                && !(status == ProductStatus::Pending || status == ProductStatus::Draft)
            {
            } else {
                let current_status = format!("${}", Product::fields().status);

                let value = match status {
                    // store user can only set the status to active
                    // only if the current status is inactive
                    ProductStatus::Active => Bson::Document(doc! {
                        "$cond": {
                            "if": {
                                "$eq": [
                                    &current_status,
                                    ProductStatus::InActive
                                ]
                            },
                            "then": ProductStatus::Active,
                            "else": current_status
                        }
                    }),
                    // The user can only set the status to inactive
                    // if the current status is active
                    ProductStatus::InActive => Bson::Document(doc! {
                        "$cond": {
                            "if": {
                                "$eq": [
                                    &current_status,
                                    ProductStatus::Active
                                ]
                            },
                            "then": ProductStatus::InActive,
                            "else": current_status
                        }
                    }),
                    // The user can only set the status to draft or pending
                    // if the current status is not deleted or banned
                    ProductStatus::Draft | ProductStatus::Pending => Bson::Document(doc! {
                        "$cond": {
                            "if": {
                                "$in": [
                                    &current_status,
                                    [
                                        ProductStatus::Deleted,
                                        ProductStatus::Banned
                                    ]
                                ]
                            },
                            "then": current_status,
                            "else": status
                        }
                    }),
                    // else the status is the current status
                    _ => Bson::String(current_status),
                };

                update.insert(Product::fields().status, value);
            }
        }

        let mut update = vec![
            doc! {
                "$set": update,
            },
            doc! {
                "$currentDate": {
                    Product::fields().updated_at: true
                }
            },
        ];

        if update_status {
            update.push(
                // Maybe this will not work
                doc! {
                    "$set": {
                        Product::fields().status: product_status_update()
                    }
                },
            );
        }

        self.find_and_update_product(filters, update, options, None)
            .await
    }

    async fn delete_product_file(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        file_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().store(true).id: store_id,
            Product::fields().assets(true).id: file_id,
            // we want to make sure that the product has at least one asset
            format!("{}.1", Product::fields().assets): {
                "$exists": true
            }
        };

        let update = doc! {
            "$pull": {
                Product::fields().assets: {
                    Product::fields().assets(false).id: file_id
                },
                Product::fields().items(true).assets_refs: file_id
            }
        };

        self.find_and_update_product(filters, update, options, None)
            .await
    }

    async fn get_products_for_store_manager(
        &self,
        store_id: &ObjectId,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        free_text: Option<String>,
        category_id: Option<ObjectId>,
        status: Option<ProductStatus>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();
        let sorting = sorting.unwrap_or_default();

        let sort_stage = match sorting.sort_by {
            ProductSortBy::Date => aggregations::sort(doc! {
                Product::fields().created_at: &sorting.direction
            }),
            ProductSortBy::Popularity => aggregations::sort(doc! {
                Product::fields().analytics(true).views: &sorting.direction
            }),
            ProductSortBy::Relevance => {
                if free_text.is_some() {
                    aggregations::sort(doc! {
                        "score": &sorting.direction
                    })
                } else {
                    aggregations::sort(doc! {
                        Product::fields().created_at: -1
                    })
                }
            }
        };

        let filters = {
            let mut f = vec![doc! {
                "equals": {
                    "value": store_id,
                    "path": Product::fields().store(true).id
                },
                "text": {
                    "query": [ProductStatus::Active, ProductStatus::Draft, ProductStatus::Pending, ProductStatus::InActive, ProductStatus::Banned],
                    "path": Product::fields().status
                }
            }];

            if let Some(category_id) = category_id {
                f.push(doc! {
                    "equals": {
                        "value": category_id,
                        "path": Product::fields().categories(true).ids
                    }
                });
            }

            if let Some(status) = status {
                f.push(doc! {
                    "text": {
                        "query": status,
                        "path": Product::fields().status
                    }
                });
            }

            f
        };

        let pipeline = [
            aggregations::search_products(&free_text, &filters, Some(1)),
            aggregations::add_score_meta(),
            sort_stage,
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::add_fields(doc! {
                Product::fields().items: {
                    "$first": format!("${}", Product::fields().items)
                },
                "total_items": {
                    "$size": format!("${}", Product::fields().items)
                }
            }),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    "total_items",
                    Product::fields().created_at,
                    Product::fields().brand,
                    Product::fields().name,
                    Product::fields().description,
                    Product::fields().keywords,
                    Product::fields().store,
                    Product::fields().categories,
                    Product::fields().variants,
                    Product::fields().analytics(true).views,
                    // Product items fields to return
                    Product::fields().items(true).id,
                    Product::fields().items(true).price,
                    Product::fields().items(true).in_storage,
                    Product::fields().items(true).variants,
                    Product::fields().items(true).name,
                    Product::fields().items(true).assets_refs,
                    Product::fields().items(true).sku,
                    Product::fields().items(true).info,
                    Product::fields().items(true).status,
                    // Product assets fields to return
                    Product::fields().assets(true).id,
                    Product::fields().assets(true).file_name,
                    Product::fields().assets(true).path,
                    Product::fields().assets(true).size,
                    Product::fields().assets(true).mime_type,
                    Product::fields().assets(true).file_type,
                ],
                None,
            ),
        ];

        let products = self
            .aggregate_products(pipeline, options.clone(), None)
            .await?;

        let count = products.len();

        if !pagination.need_count(count) {
            return Ok((products, pagination.calculate_total(count)));
        }

        let count = self
            .count_products_with_aggregation(
                [
                    aggregations::search_products(&free_text, &filters, Some(1)),
                    aggregations::count("count"),
                ],
                options,
                None,
            )
            .await?;

        Ok((products, count))
    }

    async fn delete_product_item(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().status: {
                "$nin": [
                    ProductStatus::Deleted,
                    ProductStatus::Banned,
                ]
            },
            Product::fields().store(true).id: store_id,
            Product::fields().items(true).id: item_id
        };

        let update = doc! {
            "$set": {
                format!("{}.$.{}", Product::fields().items, Product::fields().items(false).status): ProductItemStatus::Deleted
            },
            "$currentDate": {
                format!("{}.$.{}", Product::fields().items, Product::fields().items(false).updated_at): true
            }
        };

        self.update_product(filters, update, options, None).await
    }
}

fn product_status_update() -> Document {
    doc! {
        "$cond": {
            "if": {
                "$in": [
                    format!("${}", Product::fields().status),
                    [
                        ProductStatus::Active,
                        ProductStatus::InActive,
                    ]
                ]
            },
            "then": ProductStatus::Pending,
            "else": format!("${}", Product::fields().status)
        }
    }
}
