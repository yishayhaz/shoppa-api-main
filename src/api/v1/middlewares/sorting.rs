use crate::{db::Sorter, helpers::types::MyOption};
use std::str::FromStr;
use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::Response,
};
use serde::de::DeserializeOwned;

pub type OptionalSorting<T> = MyOption<Sorter<T>>;

#[async_trait]
impl<S, T> FromRequestParts<S> for OptionalSorting<T>
where
    S: Sync + Send,
    T: DeserializeOwned + FromStr + Sync + Send
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let res = Query::<Sorter<T>>::from_request_parts(parts, state).await;

        if let Ok(res) = res {
            return Ok(MyOption::Some(res.0));
        }

        Ok(MyOption::None)
    }
}
