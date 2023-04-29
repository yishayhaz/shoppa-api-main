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

pub async fn get_variants_for_extarnel(db: &DBExtension) -> PaginatedResult<Document> {
    let pipeline = [aggregations::project(
        ProjectIdOptions::Keep,
        vec![Variants::fields().name],
        None,
    )];

    let cursor = db
        .variants
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    let variants = consume_cursor(cursor)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    let count = variants.len() as u64;

    Ok((variants, count))
}
