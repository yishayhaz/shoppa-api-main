use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Bson, Document};
use mongodb::{
    options::{AggregateOptions, FindOneAndUpdateOptions, FindOneOptions, UpdateOptions},
    results::UpdateResult,
};
use serde::Deserialize;
use shoppa_core::{
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{
            EmbeddedDocument, FileDocument, Order, ProducdBrandField, Product, ProductItemStatus,
            ProductStatus, Store, Variants,
        },
        populate::ProductsPopulate,
        DBConection, Pagination, Sorter,
    },
    parser::FieldPatch,
    random,
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
    async fn get_products_count(
        &self,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
    ) -> Result<u64>;
    async fn update_products_storage_by_order(
        &self,
        order: &Order,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;
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

    async fn get_product_by_id_and_store_id(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<ProductsPopulate>,
    ) -> Result<Option<Product>>;

    async fn get_product_by_id_for_store_manager(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>>;
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
        let filters = vec![
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
                    "filter": filters
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
            aggregations::lookup::<Store>("store._id", "_id", "store", None, None),
            aggregations::unwind("store", false),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    Product::fields().created_at,
                    Product::fields().brand,
                    Product::fields().name,
                    Product::fields().description,
                    Product::fields().keywords,
                    Product::fields().categories,
                    Product::fields().variants,
                    Product::fields().analytics,
                    Product::fields().feature_bullet_points,
                    Product::fields().warranty,
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
                    Product::fields().store(true).name,
                    Product::fields().store(true).id,
                    "store.delivery_strategies",
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
            // In the future return the most relevant item
            aggregations::add_fields(doc! {
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
                    // Product items fields to return
                    format!("item.{}", Product::fields().items(false).id).as_str(),
                    format!("item.{}", Product::fields().items(false).price).as_str(),
                    format!("item.{}", Product::fields().items(false).in_storage).as_str(),
                    format!("item.{}", Product::fields().items(false).variants).as_str(),
                    format!("item.{}", Product::fields().items(false).name).as_str(),
                    format!("item.{}", Product::fields().items(false).assets_refs).as_str(),
                    format!("item.{}", Product::fields().items(false).sku).as_str(),
                    format!("item.{}", Product::fields().items(false).info).as_str(),
                    format!("item.{}", Product::fields().items(false).status).as_str(),
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
                            "field": Product::fields().items(true).id,
                            "input": aggregations::random_array_element(&format!("${}", Product::fields().items)),
                        }
                    },
                    "views": format!("${}", Product::fields().analytics(true).views),
                }),
            ),
        ];

        self.aggregate_products(pipeline, options, None).await
    }

    async fn get_products_count(
        &self,
        store_id: Option<ObjectId>,
        category_id: Option<ObjectId>,
    ) -> Result<u64> {
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
            aggregations::count("count"),
        ];

        self.count_products_with_aggregation(pipeline, None, None)
            .await
    }

    async fn update_products_storage_by_order(
        &self,
        order: &Order,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        let mut products = Vec::new();

        let mut items_quantity = vec![];

        order.parts.iter().for_each(|part| {
            part.items.iter().for_each(|item| {
                products.push(item.product_id());
                items_quantity.push(doc! {
                    "_id": &item.item_id,
                    "quantity": item.quantity
                });
            });
        });

        let filter_update_items = aggregations::filter(
            items_quantity,
            "update_item",
            doc! {
                "$eq": [
                    "$$update_item._id",
                    format!("$$item.{}", Product::fields().items(false).id)
                ]
            },
        );

        let subtract_item_in_storage = doc! {
            "$subtract": [
                format!("$$item.{}", Product::fields().items(false).in_storage),
                {
                    "$getField": {
                        "field": "quantity",
                        "input": {
                            "$arrayElemAt": [
                                &filter_update_items,
                                0
                            ]
                        }
                    }
                }
            ]
        };

        let update = vec![doc! {
            "$set": {
                Product::fields().items: aggregations::map(
                    format!("${}", Product::fields().items),
                    "item",
                    doc! {
                        "$cond": {
                            "if": {
                                "$gt": [
                                    {
                                        "$size": &filter_update_items
                                    },
                                    0
                                ]
                            },
                            "then": {
                                "$mergeObjects": [
                                    "$$item",
                                    {
                                        Product::fields().items(false).in_storage: {
                                            "$cond": {
                                                "if": {
                                                    "$gt": [
                                                        &subtract_item_in_storage,
                                                        0
                                                    ]
                                                },
                                                "then": subtract_item_in_storage,
                                                "else": 0
                                            }
                                        }
                                    }
                                ]
                            },
                            "else": "$$item"
                        }
                    }
                )
            }
        }];

        let filters = doc! {
            Product::fields().id: {
                "$in": products
            }
        };

        self.update_many_products(filters, update, options, None)
            .await
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
                format!("{}.$[].{}", Product::fields().items, Product::fields().items(false).assets_refs): file_id
            }
        };

        self.find_and_update_product(filters, update, options, None)
            .await
    }

    async fn get_products_for_admins(
        &self,
        pagination: Option<Pagination>,
        sorting: Option<Sorter<ProductSortBy>>,
        product_name: Option<String>,
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
                if product_name.is_some() {
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

        let search_stage = {
            if filters.is_empty() && product_name.is_none() {
                aggregations::match_all()
            } else {
                aggregations::product_name_search(product_name, filters)
            }
        };

        let pipeline = [
            search_stage.clone(),
            aggregations::add_score_meta(),
            sort_stage,
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::add_fields(doc! {
                Product::fields().items: {
                    "$arrayElemAt": [
                        format!("${}", Product::fields().items),
                        0
                    ]
                },
                "total_items": {
                    "$size": format!("${}", Product::fields().items)
                },
            }),
            aggregations::project(
                ProjectIdOptions::Keep,
                vec![
                    "total_items",
                    Product::fields().name,
                    Product::fields().analytics,
                    Product::fields().categories,
                    Product::fields().created_at,
                    Product::fields().store,
                    Product::fields().assets,
                    Product::fields().status,
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
                [search_stage, aggregations::count("count")],
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
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let update = vec![doc! {
            "$set": {
                Product::fields().updated_at: "$$NOW",
                Product::fields().status: product_status_update(),
                Product::fields().assets: {
                    "$concatArrays": [
                        format!("${}", Product::fields().assets),
                        [asset.into_bson()?]
                    ]
                }
            }
        }];

        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().store(true).id: store_id,
            Product::fields().status: {
                "$nin": [ProductStatus::Deleted, ProductStatus::Banned]
            }
        };

        self.find_and_update_product(filters, update, options, None)
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
                    Product::fields().items(false).id: &item_id,
                    Product::fields().items(false).status: {
                        "$ne": ProductItemStatus::Deleted
                    }
                }
            },
            Product::fields().status: {
                "$nin": [ProductStatus::Deleted, ProductStatus::Banned]
            }
        };

        let mut set_doc = Document::new();

        if let Some(price) = price {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).price
            );
            set_doc.insert(field, price);
        }

        if let Some(in_storage) = in_storage {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).in_storage
            );
            set_doc.insert(field, in_storage as i64);
        }

        if FieldPatch::Missing != name {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).name
            );
            set_doc.insert(field, name.into_option());
        }

        if let Some(assets_refs) = assets_refs {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).assets_refs
            );
            set_doc.insert(field, assets_refs);
        }

        if FieldPatch::Missing != sku {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).sku
            );
            set_doc.insert(field, sku.into_option());
        }

        if FieldPatch::Missing != info {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).info
            );
            set_doc.insert(field, info.into_option());
        }

        if let Some(status) = status {
            let field = format!(
                "{}.$.{}",
                Product::fields().items,
                Product::fields().items(false).status
            );
            set_doc.insert(field, status);
        }

        if set_doc.is_empty() {
            return Err(Error::Static("No update fields provided"));
        }

        let update = doc! {
            "$set": set_doc,
            "$currentDate": {
                format!("{}.$.{}", Product::fields().items, Product::fields().items(false).updated_at): true
            }
        };

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
                    // The user can only set the status to inactive
                    // if the current status is active
                    ProductStatus::Inactive => Bson::Document(doc! {
                        "$cond": {
                            "if": {
                                "$in": [
                                    &current_status,
                                    [ProductStatus::Active, ProductStatus::Pending, ProductStatus::Draft]
                                ]
                            },
                            "then": ProductStatus::Inactive,
                            "else": current_status
                        }
                    }),
                    ProductStatus::Pending => Bson::Document(doc! {
                        "$cond": {
                            "if": {
                                "$in": [
                                    &current_status,
                                    [
                                        ProductStatus::Inactive,
                                        ProductStatus::Draft
                                    ]
                                ]
                            },
                            "then": status,
                            "else": current_status
                        }
                    }),
                    ProductStatus::Deleted => Bson::Document(doc! {
                        "$cond": {
                            "if": {
                                "$in": [
                                    &current_status,
                                    [
                                        ProductStatus::Inactive,
                                        ProductStatus::Draft,
                                        ProductStatus::Pending,
                                        ProductStatus::Active
                                    ]
                                ]
                            },
                            "then": status,
                            "else": current_status
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
                "$set": {
                    Product::fields().updated_at: "$$NOW"
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
                format!("{}.$[].{}", Product::fields().items, Product::fields().items(false).assets_refs): file_id
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
        product_name: Option<String>,
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
                if product_name.is_some() {
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
                "equals": {
                    "value": store_id,
                    "path": Product::fields().store(true).id
                }},
                doc! {"text": {
                        "query": [ProductStatus::Active, ProductStatus::Draft, ProductStatus::Pending, ProductStatus::Inactive, ProductStatus::Banned],
                        "path": Product::fields().status
                    }
                },
            ];

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

        let search_stage = aggregations::product_name_search(product_name, filters);

        let pipeline = [
            search_stage.clone(),
            aggregations::add_score_meta(),
            sort_stage,
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::add_fields(doc! {
                "filterd_items": {
                    "$filter": {
                        "input": format!("${}", Product::fields().items),
                        "as": "item",
                        "cond": {
                            "$ne": [
                                format!("$$item.{}", Product::fields().items(false).status),
                                ProductItemStatus::Deleted
                            ]
                        }
                    }
                }
            }),
            aggregations::add_fields(doc! {
                Product::fields().items: {
                    "$arrayElemAt": [
                        "$filterd_items",
                        0
                    ]
                },
                "total_items": {
                    "$size": "$filterd_items"
                }
            }),
            // aggregations::lookup::<Variants>(
            //     Product::fields().items(true).variants,
            //     Variants::fields().id,
            //     Product::fields().items(true).variants,
            //     None,
            //     None,
            // ),
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
                    Product::fields().status,
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
                [search_stage, aggregations::count("count")],
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

    async fn get_product_by_id_and_store_id(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<ProductsPopulate>,
    ) -> Result<Option<Product>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().status: {
                "$nin": [
                    ProductStatus::Deleted,
                    ProductStatus::Banned,
                ]
            },
            Product::fields().store(true).id: store_id
        };

        self.get_product(filters, options, populate, None).await
    }

    async fn get_product_by_id_for_store_manager(
        &self,
        product_id: &ObjectId,
        store_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>> {
        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().status: {
                "$ne": ProductStatus::Deleted
            },
            Product::fields().store(true).id: store_id
        };

        let pipeline = [
            aggregations::match_query(&filters),
            aggregations::lookup_product_variants(None),
            aggregations::add_fields(doc! {
                Product::fields().items: {
                    "$filter": {
                        "input": format!("${}", Product::fields().items),
                        "as": "item",
                        "cond": {
                            "$ne": [
                                format!("$$item.{}", Product::fields().items(false).status),
                                ProductItemStatus::Deleted
                            ]
                        }
                    }
                }
            }),
            aggregations::unset(vec![Product::fields().store]),
        ];

        Ok(self
            .aggregate_products(pipeline, options, None)
            .await?
            .pop())
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
                        ProductStatus::Inactive,
                    ]
                ]
            },
            "then": ProductStatus::Pending,
            "else": format!("${}", Product::fields().status)
        }
    }
}

pub fn generate_products_random_sort() -> Document {
    let mut fields = vec![
        Product::fields().analytics(true).views,
        Product::fields().assets(true).size,
        Product::fields().items(true).price,
        Product::fields().items(true).in_storage,
        Product::fields().created_at,
        Product::fields().updated_at,
        Product::fields().name,
        Product::fields().brand(true).name,
        Product::fields().store(true).id,
        Product::fields().warranty,
    ];

    let total_sorts = random::random_number_from_range(2, fields.len() as u32);

    let mut sorts = doc! {};

    for _ in 0..total_sorts {
        let field =
            fields.remove(random::random_number_from_range(0, fields.len() as u32) as usize);
        let order = random::random_number_from_range(0, 2) as i32;

        if order == 0 {
            sorts.insert(field, -1);
            continue;
        }

        sorts.insert(field, order);
    }

    sorts
}
