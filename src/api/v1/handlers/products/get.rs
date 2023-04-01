use super::super::prelude::routes::*;
use crate::db::queries;

pub async fn get_products(db: DBExtension) -> HandlerResponse {
    let products = queries::get_products_for_extarnel(&db).await?;

    Ok(ResponseBuilder::success(Some(products), None, None).into_response())
}
