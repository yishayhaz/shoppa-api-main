use crate::helpers::{extractors::QueryWithValidation, parser::empty_string_as_none};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use validator::Validate;

pub struct Pagination {
    pub page: u32,
    pub amount: u32,
    pub offset: u32,
}


#[derive(Debug, Validate, Deserialize)]
struct PaginationPrivate {
    #[serde(deserialize_with = "empty_string_as_none")]
    page: Option<u32>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[validate(range(max=100))]
    amount: Option<u32>,
}

// source https://github.com/imbolc/axum-client-ip/
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

        let page = v.page.unwrap_or(0);
        let amount = v.amount.unwrap_or(10);

        Ok(Pagination{page, amount, offset: page * amount})
    }
}
