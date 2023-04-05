use crate::{
    db::Pagination,
    helpers::{extractors::QueryWithValidation, parser::empty_string_as_none},
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use validator::Validate;

const MAX_AMOUNT: i64 = 100;
// To make sure the offset isn't bigger then the i64 MAX value
const MAX_PAGE: i64 = i64::MAX / MAX_AMOUNT;

#[derive(Debug, Validate, Deserialize)]
struct PaginationPrivate {
    #[serde(deserialize_with = "empty_string_as_none")]
    #[validate(range(min = 0, max = "MAX_PAGE"))]
    page: Option<i64>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[validate(range(min = 1, max = "MAX_AMOUNT"))]
    amount: Option<i64>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Pagination
where
    S: Sync + Send,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let QueryWithValidation(v) =
            QueryWithValidation::<PaginationPrivate>::from_request_parts(parts, state)
                .await
                .map_err(|op| op.into_response())?;

        let default = Pagination::default();

        let page = v.page.unwrap_or(default.page);
        let amount = v.amount.unwrap_or(default.amount);

        Ok(Pagination {
            page,
            amount,
            offset: page * amount,
        })
    }
}
