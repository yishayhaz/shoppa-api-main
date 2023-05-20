use super::prelude::*;
use crate::prelude::*;

type GetStoreResult = Result<Option<models::Store>>;
// Stores as many stores
type GetStoresResult = Result<Vec<models::Store>>;

pub async fn get_stores(db: &DBExtension) -> GetStoresResult {
    let cursor = db
        .stores
        .find(None, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    let stores = cursor.consume().await?;

    Ok(stores)
}

async fn get_store(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetStoreResult {
    let store = db
        .stores
        .find_one(filter, option)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(store)
}

pub async fn get_store_by_id(db: &DBExtension, id: &ObjectId) -> GetStoreResult {
    let filter = doc! {
        "_id": id,
    };

    get_store(db, filter, None).await
}

pub async fn get_stores_count(db: &DBExtension) -> Result<u64> {
    let count = db
        .stores
        .count_documents(None, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(count)
}

pub async fn get_stores_names_for_autocomplete(
    db: &DBExtension,
    free_text: String,
) -> Result<Vec<Document>> {
    let cursor = db
        .stores
        .aggregate(
            [
                aggregations::autocomplete_store_search(&free_text),
                aggregations::add_fields(doc! {
                    "score": {
                        "$meta": "searchScore"
                    }
                }),
                aggregations::sort(doc! {
                    "score": -1
                }),
                aggregations::limit(3),
                aggregations::project(ProjectIdOptions::Keep, [models::Store::fields().name], None),
            ],
            None,
        )
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(cursor.consume().await?)
}

pub async fn get_stores_for_extarnel(
    db: &DBExtension,
    pagination: Option<Pagination>,
    free_text: Option<String>,
) -> PaginatedResult<Document> {
    let pagination = pagination.unwrap_or_default();

    let pipeline = [
        aggregations::search_store(&free_text, &vec![], None),
        aggregations::add_fields(doc! {
            "score": {
                "$meta": "searchScore"
            }
        }),
        aggregations::sort(doc! {
            "score": -1
        }),
        aggregations::skip(pagination.offset),
        aggregations::limit(pagination.amount),
        aggregations::project(
            ProjectIdOptions::Keep,
            [
                models::Store::fields().name,
                models::Store::fields().logo(true).path,
                models::Store::fields().logo(true).file_name,
                models::Store::fields().logo(true).mime_type,
                models::Store::fields().logo(true).file_type,
                models::Store::fields().banner(true).path,
                models::Store::fields().banner(true).file_name,
                models::Store::fields().banner(true).mime_type,
                models::Store::fields().banner(true).file_type,
                models::Store::fields().description,
                models::Store::fields().slogan,
            ],
            None,
        ),
    ];

    let cursor = db
        .stores
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    let stores = cursor.consume().await?;

    let mut count = stores.len() as i64;

    if count < pagination.amount {
        count += pagination.offset;

        return Ok((stores, count as u64));
    }

    let cursor = db
        .stores
        .aggregate(
            [
                aggregations::search_store(&free_text, &vec![], None),
                aggregations::count("count"),
            ],
            None,
        )
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok((stores, cursor.extract_count().await?))
}
