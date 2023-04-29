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

        let product = convert_one_doc_cursor::<Product>(cursor)
            .await
            .map_err(|e| Error::DBError(("products", e)))?;

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
                    "score": { "$meta": "textScore" }
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
                            "score": { "$meta": "textScore" }
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

    let query = match infinite {
        true => {
            let mut q = match free_text {
                Some(text) => doc! {
                   "$or": [
                        {"$text": {"$search": text}}, {"_id": {"$exists": true}}
                   ]
                },
                None => doc! {},
            };
            if let Some(store_id) = store_id {
                q.insert("store._id", store_id);
            }

            if let Some(category_id) = category_id {
                q.insert(
                    "categories._id",
                    doc! {
                    "$in": [category_id]},
                );
            }
            q
        }
        false => {
            let mut q = match free_text {
                Some(text) => doc! {
                    "$text": {"$search": text}
                },
                None => doc! {},
            };

            if let Some(store_id) = store_id {
                q.insert("store._id", store_id);
            }

            if let Some(category_id) = category_id {
                q.insert(
                    "categories._id",
                    doc! {
                    "$in": [category_id]},
                );
            }

            q
        }
    };

    let pipeline = [
        aggregations::match_query(&query),
        sort_stage,
        aggregations::skip(pagination.offset),
        aggregations::limit(pagination.amount),
        aggregations::project(
            ProjectIdOptions::ToString,
            vec![
                Product::fields().brand,
                Product::fields().name,
                Product::fields().keywords,
                "store.name",
                "analytics",
            ],
            Some(doc! {
                Product::fields().categories: {
                "$map": {
                    "input": "$categories",
                    "in": {
                        "_id":{"$toString": "$$this._id"},
                        "name": "$$this.name"
                    }
                    }
                },
                "store._id": aggregations::convert_to_string_safe("$store._id"),
                Product::fields().created_at: aggregations::convert_to_string_safe("$created_at")
            }),
        ),
    ];

    let cursor = db
        .products
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    let products = consume_cursor(cursor)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    let mut count = products.len() as i64;

    if count < pagination.amount {
        count += pagination.offset;

        return Ok((products, count as u64));
    }

    let count = db
        .products
        .count_documents(query, None)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok((products, count))
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
            ProjectIdOptions::ToString,
            ["type", "name", "values"],
            None,
        )])),
        aggregations::project(
            ProjectIdOptions::ToString,
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
                "items"
            ],
            None
        ),
    ];

    let cursor = db
        .products
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("products", e)))?;

    Ok(convert_one_doc_cursor(cursor)
        .await
        .map_err(|e| Error::DBError(("products", e)))?)
}
