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
                aggregations::add_score_meta(),
                aggregations::sort_by_score(),
                aggregations::limit(10),
                aggregations::project(ProjectIdOptions::Keep, [models::Store::fields().name], None),
            ],
            None,
        )
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(cursor.consume().await?)
}

pub async fn get_random_stores_names(db: &DBExtension) -> Result<Vec<Document>> {
    let cursor = db
        .stores
        .aggregate(
            [
                aggregations::sample(10),
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
        aggregations::add_score_meta(),
        aggregations::sort_by_score(),
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
                models::Store::fields().created_at,
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

pub async fn get_stores_for_admins(
    db: &DBExtension,
    pagination: Option<Pagination>,
) -> PaginatedResult<Document> {
    let pagination = pagination.unwrap_or_default();

    let pipeline = [
        aggregations::skip(pagination.offset),
        aggregations::limit(pagination.amount),
        aggregations::project(
            ProjectIdOptions::Keep,
            [
                models::Store::fields().name,
                models::Store::fields().created_at,
                models::Store::fields().analytics,
                models::Store::fields().contact,
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

    let count = stores.len();

    if !pagination.need_count(count) {
        return Ok((stores, pagination.calculate_total(count)));
    }

    let count = db
        .stores
        .count_documents(doc! {}, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok((stores, count))
}
