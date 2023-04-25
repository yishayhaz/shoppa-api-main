use crate::{db::queries, prelude::{handlers::*, *}};

pub async fn get_root_categories(db: DBExtension) -> HandlerResponse {
    let categories = queries::get_categories_for_extarnel(&db, None, None).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}

pub async fn get_inner_categories(
    db: DBExtension,
    Path(parent_id): Path<ObjectId>,
) -> HandlerResponse {
    let categories = queries::get_categories_for_extarnel(&db, Some(parent_id), None).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}

pub async fn get_inner_inner_categories(
    db: DBExtension,
    Path((parent_parent_id, parent_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResponse {
    let categories =
        queries::get_categories_for_extarnel(&db, Some(parent_parent_id), Some(parent_id)).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}
