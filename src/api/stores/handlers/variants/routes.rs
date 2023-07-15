use super::types;
use crate::{
    db::{AxumDBExtansion, StoreUserVariantsFunctions},
    prelude::*,
};
use axum::{extract::Query, response::IntoResponse};
use shoppa_core::{db::Pagination, extractors::QueryWithValidation, ResponseBuilder};

pub async fn get_variants_by_ids(
    db: AxumDBExtansion,
    QueryWithValidation(query): QueryWithValidation<types::GetVariantsByIdsQuery>,
) -> HandlerResult {
    let variants = db.get_variants_by_ids(&query.variants_ids).await?;

    Ok(ResponseBuilder::success(Some(variants), None, None).into_response())
}

pub async fn autocomplete_variants(
    db: AxumDBExtansion,
    pagination: Pagination,
    Query(query): Query<types::GetVariantsAutocompleteQuery>,
) -> HandlerResult {
    let variants = db
        .autocomplete_variants_search(Some(pagination), query.categories_ids, query.free_text)
        .await?;

    Ok(ResponseBuilder::success(Some(variants), None, None).into_response())
}
