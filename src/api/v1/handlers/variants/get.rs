use crate::{
    db::queries,
    prelude::{handlers::*, *}, api::v1::middlewares::OnlyInDev,
};
use super::types;

pub async fn get_variants(db: DBExtension, _: OnlyInDev) -> HandlerResult {
    let variants = queries::get_variants_for_extarnel(&db).await?;

    Ok(ResponseBuilder::paginated_response(&variants).into_response())
}

pub async fn get_variants_by_ids(
    db: DBExtension,
    Query(query): Query<types::GetVariantsByIdsQuery>,
)  -> HandlerResult{
    // todo: this doesnt read ?ids=[2,3,4,5]

    let variants = queries::get_variants_by_ids(&db, &query.variants_ids).await?;

    Ok(ResponseBuilder::success(Some(variants), None, None).into_response())
}

pub async fn get_variant_by_id(
    db: DBExtension,
    Path(variant_id): Path<ObjectId>,
) -> HandlerResult {
    let variant = queries::get_variant_by_id(&db, &variant_id).await?;

    Ok(ResponseBuilder::success(Some(variant), None, None).into_response())
}