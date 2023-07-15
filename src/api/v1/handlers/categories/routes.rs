use super::types::GetCategoriesQueryParams;
use crate::{
    db::{AxumDBExtansion, CategoriesFunctions},
    prelude::*,
};
use axum::{extract::Query, response::IntoResponse};
use shoppa_core::ResponseBuilder;

pub async fn get_catagories(
    db: AxumDBExtansion,
    Query(query): Query<GetCategoriesQueryParams>,
) -> HandlerResult {
    let categories = db.get_categories_for_external(query.parent).await?;

    Ok(ResponseBuilder::success(Some(categories), None, None).into_response())
}
