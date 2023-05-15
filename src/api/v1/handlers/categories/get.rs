use crate::{db::queries, prelude::{handlers::*, *}};
use super::types;

pub async fn get_root_categories(db: DBExtension) -> HandlerResult {
    let categories = queries::get_categories_for_extarnel(&db, None, None).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}

pub async fn get_inner_categories(
    db: DBExtension,
    Path(parent_id): Path<ObjectId>,
) -> HandlerResult {
    let categories = queries::get_categories_for_extarnel(&db, Some(parent_id), None).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}

pub async fn get_inner_inner_categories(
    db: DBExtension,
    Path((parent_parent_id, parent_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResult {
    let categories =
        queries::get_categories_for_extarnel(&db, Some(parent_parent_id), Some(parent_id)).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}

pub async fn get_category_info(
    db: DBExtension,
    // TODO: read ids from params
    Path(payload): Path<types::GetCategoryInfo>,
) -> HandlerResult {
    let category = queries::get_category_by_ids(&db, &payload.category_ids).await?;

    Ok(ResponseBuilder::success(Some(category), None, None).into_response())
}

