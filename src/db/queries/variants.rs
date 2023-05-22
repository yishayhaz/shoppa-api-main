use super::prelude::*;
use crate::prelude::*;
use models::Variants;

type _GetVariantResult = Result<Option<Variants>>;

async fn _get_variant(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> _GetVariantResult {
    let variant = db
        .variants
        .find_one(filter, option)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    Ok(variant)
}

pub async fn validate_many_variants_exist(
    db: &DBExtension,
    variants_ids: &Vec<ObjectId>,
) -> Result<bool> {
    let count = db
        .variants
        .count_documents(
            doc! {
                Variants::fields().id: {
                    "$in": variants_ids
                }
            },
            None,
        )
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    Ok(count == variants_ids.len() as u64)
}

pub async fn get_variants_for_extarnel(
    db: &DBExtension,
    pagination: Option<Pagination>,
) -> PaginatedResult<Document> {
    let pagination = pagination.unwrap_or_default();

    let pipeline = [
        aggregations::skip(pagination.offset),
        aggregations::limit(pagination.amount),
        aggregations::project(
            ProjectIdOptions::Keep,
            vec![Variants::fields().name, Variants::fields().values, "type"],
            None,
        ),
    ];

    let cursor = db
        .variants
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    let variants = cursor.consume().await?;

    let mut count = variants.len() as i64;

    if count < pagination.amount {
        count += pagination.offset;

        return Ok((variants, count as u64));
    }

    let count = db
        .variants
        .count_documents(doc! {}, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    Ok((variants, count))
}

pub async fn get_variants_by_ids(
    db: &DBExtension,
    variants_ids: &Vec<ObjectId>,
) -> Result<Vec<Document>> {
    let pipeline = [
        aggregations::match_query(&doc! {
            Variants::fields().id: {
                "$in": variants_ids
            }
        }),
        aggregations::project(
            ProjectIdOptions::Keep,
            vec![Variants::fields().name, Variants::fields().values, "type"],
            None,
        ),
    ];

    let cursor = db
        .variants
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    let variants = cursor.consume().await?;

    Ok(variants)
}

pub async fn get_variant_by_id(db: &DBExtension, variant_id: &ObjectId) -> _GetVariantResult {
    let filters = doc! {
        Variants::fields().id: variant_id
    };

    _get_variant(db, filters, None).await
}
