use super::prelude::*;
use models::Variants;

type _GetVariantResult = Result<Option<Variants>, Response>;

async fn _get_variant(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> _GetVariantResult {
    let variant = db.variants.find_one(filter, option).await.map_err(|e| {
        ResponseBuilder::query_error(Variants::get_collection_name(), e).into_response()
    })?;

    Ok(variant)
}

pub async fn validate_many_variants_exist(
    db: &DBExtension,
    variants_ids: &Vec<ObjectId>,
) -> Result<bool, Response> {
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
        .map_err(|_| {
            ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while checking variants"),
                Some(500),
            )
            .into_response()
        })?;

    Ok(count == variants_ids.len() as u64)
}

pub async fn get_variants_for_extarnel(db: &DBExtension) -> PaginatedResult<Document> {

    let pipeline = [
        aggregations::project(ProjectIdOptions::ToString, vec![Variants::fields().name], None)
    ];

    let cursor = db.variants.aggregate(pipeline, None).await.map_err(|e| {
        ResponseBuilder::query_error(Variants::get_collection_name(), e).into_response()
    })?;

    let variants = consume_cursor(cursor).await.map_err(|e| {
        ResponseBuilder::cursor_consumpetion_error(Variants::get_collection_name(), e)
            .into_response()
    })?;

    let count = variants.len() as u64;

    Ok((variants, count))
}
