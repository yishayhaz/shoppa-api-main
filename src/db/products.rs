use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{AggregateOptions, FindOneAndUpdateOptions};
use serde::Deserialize;
use shoppa_core::{
    constans,
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{
            EmbeddedDocument, FileDocument, ProducdBrandField, Product, ProductItemStatus,
            ProductStatus, Variants,
        },
        DBConection, Pagination, Sorter,
    },
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
        name: Option<String>,
        images_refs: Option<Vec<ObjectId>>,
        sku: Option<String>,
        info: Option<String>,
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
                    "query": [
                        ProductItemStatus::Active,
                        ProductItemStatus::SoldOut
                    ]
                }

            },
            doc! {
                "text": {
                    "path": Product::fields().items(true).status,
                    "query": [
                        ProductItemStatus::Active,
                        ProductItemStatus::SoldOut
                    ]
                }
            }
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
                    "query": [
                        ProductItemStatus::Active,
                        ProductItemStatus::SoldOut
                    ]
                }

            },
            doc! {
                "text": {
                    "path": Product::fields().items(true).status,
                    "query": [
                        ProductItemStatus::Active,
                        ProductItemStatus::SoldOut
                    ]
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
                            "$in": [
                                format!("$$item.{}", Product::fields().items(false).status),
                                [ProductItemStatus::Active, ProductItemStatus::SoldOut]
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
                        "query": [
                            ProductItemStatus::Active,
                            ProductItemStatus::SoldOut
                        ]
                    }
    
                },
                doc! {
                    "text": {
                        "path": Product::fields().items(true).status,
                        "query": [
                            ProductItemStatus::Active,
                            ProductItemStatus::SoldOut
                        ]
                    }
                }
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
                [aggregations::search_products(&free_text, &filters, Some(1))],
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
                        "query": [
                            ProductItemStatus::Active,
                            ProductItemStatus::SoldOut
                        ]
                    }
    
                },
                doc! {
                    "text": {
                        "path": Product::fields().items(true).status,
                        "query": [
                            ProductItemStatus::Active,
                            ProductItemStatus::SoldOut
                        ]
                    }
                }
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
        name: Option<String>,
        assets_refs: Option<Vec<ObjectId>>,
        sku: Option<String>,
        info: Option<String>,
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

        if let Some(name) = name {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).name
            );
            if name == constans::DELETE_FIELD_KEY_OPETATOR {
                update.insert::<_, Option<String>>(field, None);
            } else {
                update.insert(field, name);
            }
        }

        if let Some(assets_refs) = assets_refs {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).assets_refs
            );
            update.insert(field, assets_refs);
        }

        if let Some(sku) = sku {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).sku
            );
            if sku == constans::DELETE_FIELD_KEY_OPETATOR {
                update.insert::<_, Option<String>>(field, None);
            } else {
                update.insert(field, sku);
            }
        }

        if let Some(info) = info {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).info
            );
            if info == constans::DELETE_FIELD_KEY_OPETATOR {
                update.insert::<_, Option<String>>(field, None);
            } else {
                update.insert(field, info);
            }
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
                }
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
                [aggregations::search_products(&free_text, &filters, Some(1))],
                options,
                None,
            )
            .await?;

        Ok((products, count))
    }
}
