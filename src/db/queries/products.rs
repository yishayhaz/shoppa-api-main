use super::prelude::*;
use crate::{
    db::populate::{PopulateOptions, ProductsPopulate},
    prelude::*,
};
use models::{Product, ProductSortBy};

type GetProductResult = Result<Option<Product>>;

async fn get_product(
    db: &DBExtension,
    filter: Document,
    populate: Option<ProductsPopulate>,
    option: Option<FindOneOptions>,
) -> GetProductResult {
    if let Some(populate) = populate {
        let mut pipeline = vec![aggregations::match_query(&filter), aggregations::limit(1)];

        pipeline.extend(populate.build_pipeline());

        let cursor = db
            .products
            .aggregate(pipeline, None)
            .await
            .map_err(|e| Error::DBError(("products", e)))?;

        let product = cursor
            .convert_one_doc::<Product>()
            .await?;

        return Ok(product);
    };

    let product = db
        .products
        .find_one(filter, option)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok(product)
}

pub async fn get_product_by_id(
    db: &DBExtension,
    id: &ObjectId,
    populate: Option<ProductsPopulate>,
    option: Option<FindOneOptions>,
) -> GetProductResult {
    let filter = doc! {
        "_id": id,
    };

    get_product(db, filter, populate, option).await
}

pub async fn get_products_for_extarnel(
    db: &DBExtension,
    pagination: Option<Pagination>,
    sorting: Option<Sorter<models::ProductSortBy>>,
    mut free_text: Option<String>,
    store_id: Option<ObjectId>,
    category_id: Option<ObjectId>,
    infinite: bool,
) -> PaginatedResult<Document> {
    let pagination = pagination.unwrap_or_default();

    let sort_stage = match sorting {
        None => {
            if free_text.is_some() {
                aggregations::sort(doc! {
                    "score": -1
                })
            } else {
                // this is the default sorting
                aggregations::sort(doc! {
                    Product::fields().created_at: -1
                })
            }
        }
        Some(sort) => {
            let direcation = &sort.direction;
            match sort.sort_by {
                ProductSortBy::Date => {
                    // free text and infinte can only be used in Relevance sorting
                    if infinite {
                        free_text = None;
                    }
                    aggregations::sort(doc! {
                        Product::fields().created_at: direcation
                    })
                }
                ProductSortBy::Popularity => {
                    // free text and infinte can only be used in Relevance sorting
                    if infinite {
                        free_text = None;
                    }
                    aggregations::sort(doc! {
                        "analytics.views": direcation
                    })
                }
                ProductSortBy::Relevance => {
                    if free_text.is_some() {
                        aggregations::sort(doc! {
                            "score": direcation
                        })
                    } else {
                        // this is the default sorting
                        aggregations::sort(doc! {
                            Product::fields().created_at: -1
                        })
                    }
                }
            }
        }
    };

    let mut min_should_match = 1;

    if infinite {
        min_should_match = 0;
    }

    let filters = {
        let mut f = vec![];

        if let Some(store_id) = store_id {
            f.push(doc! {
                "equals": {
                    "value": store_id,
                    "path": "store._id"
                }
            });
        };

        if let Some(category_id) = category_id {
            f.push(doc! {
                "equals": {
                    "value": category_id,
                    "path": "categories._id"
                }
            });
        }
        f
    };

    let pipeline = [
        aggregations::search_products(&free_text, &filters, Some(min_should_match)),
        aggregations::add_fields(doc! {
            "score": {
                "$meta": "searchScore"
            }
        }),
        sort_stage,
        aggregations::skip(pagination.offset),
        aggregations::limit(pagination.amount),
        aggregations::project(
            ProjectIdOptions::Keep,
            vec![
                Product::fields().brand,
                Product::fields().name,
                Product::fields().keywords,
                "analytics",
                Product::fields().categories,
                Product::fields().created_at,
                Product::fields().store,
            ],
            None,
        ),
    ];

    let cursor = db
        .products
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    let products = cursor
        .consume()
        .await?;

    let mut count = products.len() as i64;

    if count < pagination.amount {
        count += pagination.offset;

        return Ok((products, count as u64));
    }

    let cursor = db
        .products
        .aggregate(
            [
                aggregations::search_products(&free_text, &filters, Some(min_should_match)),
                aggregations::count("count"),
            ],
            None,
        )
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok((products, cursor.extract_count().await?))
}

pub async fn get_one_product_for_extarnel(
    db: &DBExtension,
    id: &ObjectId,
) -> Result<Option<Document>> {
    let filter = doc! {
        "_id": id,
    };

    let pipeline = [
        aggregations::match_query(&filter),
        aggregations::lookup_product_variants(Some(vec![aggregations::project(
            ProjectIdOptions::Keep,
            ["type", "name", "values.label", "values._id", "values.value"],
            None,
        )])),
        aggregations::project(
            ProjectIdOptions::Keep,
            [
                "created_at",
                "brand",
                "name",
                "description",
                "keywords",
                "store",
                "categories.name",
                "categories._id",
                "analytics.views",
                "items",
                "variants",
            ],
            None,
        ),
    ];

    let cursor = db
        .products
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok(cursor
        .convert_one_doc()
        .await?)
}

pub async fn get_products_names_for_autocomplete(
    db: &DBExtension,
    free_text: String,
    store_id: Option<ObjectId>,
    category_id: Option<ObjectId>,
) -> Result<Vec<Document>> {
    
    let mut filters = vec![];

    if let Some(store_id) = store_id {
        filters.push(doc! {
            "equals": {
                "value": store_id,
                "path": "store._id"
            }
        });
    };

    if let Some(category_id) = category_id {
        filters.push(doc! {
            "equals": {
                "value": category_id,
                "path": "categories._id"
            }
        });
    }

    // TODO in the future we need to use the embeddeddocuments search to return the must
    // relevant product item and not the first one
    let cursor = db
        .products
        .aggregate(
            [
                aggregations::autocomplete_products_search(&free_text, filters),
                aggregations::add_fields(doc! {
                    "score": {
                        "$meta": "searchScore"
                    }
                }),
                aggregations::sort(doc! {
                    "score": -1
                }),
                aggregations::limit(10),
                aggregations::project(
                    ProjectIdOptions::Keep,
                    ["name"],
                    Some(doc! {
                        "item_id": {"$first": "$items._id"},
                        "views": "$analytics.views"
                    }),
                ),
            ],
            None,
        )
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok(cursor
        .consume()
        .await?)
}

pub async fn get_products_count(
    db: &DBExtension,
    store_id: Option<ObjectId>,
    category_id: Option<ObjectId>,
) -> Result<u64> {
    let mut query = doc! {};

    if let Some(store_id) = store_id {
        query.insert("store._id", store_id);
    }

    if let Some(category_id) = category_id {
        query.insert(
            "categories._id",
            doc! {
            "$in": [category_id]},
        );
    }

    let count = db
        .products
        .count_documents(query, None)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok(count)
}
