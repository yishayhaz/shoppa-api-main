use crate::{db::Sorter, helpers::types::MyOption};
use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::Response,
};

pub type OptionalSorting = MyOption<Sorter>;

#[async_trait]
impl<S> FromRequestParts<S> for OptionalSorting
where
    S: Sync + Send,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let res = Query::<Sorter>::from_request_parts(parts, state).await;

        if res.is_err() {
            return Ok(MyOption::None);
        }

        Ok(MyOption::Some(res.unwrap().0))
    }
}
