use super::prelude::*;
use models::Product;
use crate::db::populate::ProductsPopulate;

type GetProductResult = Result<Option<Product>, Response>;


async fn get_product(
    db: &DBExtension,
    filter: Document,
    _populate: Option<ProductsPopulate>,
    option: Option<FindOneOptions>,
) -> GetProductResult {
    let product = db.products.find_one(filter, option).await.map_err(|e| {
        ResponseBuilder::query_error(
            "products",
            e
        )
        .into_response()
    })?;

    Ok(product)
}

pub async fn get_product_by_id(db: &DBExtension, id: &ObjectId) -> GetProductResult {
    let filter = doc! {
        "_id": id,
    };

    get_product(db, filter, None, None).await
}

pub async fn get_products_for_extarnel(
    db: &DBExtension,
    pagination: Option<Pagination>,
    sorting: Option<Sorter>,
    free_text: Option<String>,
    store_id: Option<ObjectId>,
) -> PaginatedResult<Document> {
    let pagination = pagination.unwrap_or_default();

    let mut query = match &free_text {
        Some(text) => doc! {
            "$text": {"$search": text}
        },
        None => doc! {},
    };

    if store_id.is_some() {
        query.insert("store._id", store_id.unwrap());
    }

    let sort_stage = match sorting {
        None => {
            if free_text.is_some(){
                aggregations::sort(doc! {
                    "score": { "$meta": "textScore" }
                })
            }
            else{
                aggregations::sort(Sorter::default().into())
            }
        },
        Some(v) => aggregations::sort(v.into())
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

    let cursor = match db.products.aggregate(pipeline, None).await {
        Ok(v) => v,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching products"),
                Some(500),
            )
            .into_response());
        }
    };

    let products = match consume_cursor(cursor).await {
        Ok(v) => v,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching products"),
                Some(500),
            )
            .into_response());
        }
    };

    let mut count = products.len() as i64;

    if count < pagination.amount {
        count += pagination.offset;

        return Ok((products, count as u64));
    }

    let count = db
        .products
        .count_documents(query, None)
        .await
        .map_err(|_| {
            ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching products count"),
                Some(500),
            )
            .into_response()
        })?;

    Ok((products, count))
}
