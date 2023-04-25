use crate::{
    db::models,
    helpers::types::{DBExtension, ResponseBuilder},
};
use axum::response::IntoResponse;
use axum::response::Response;
use bson::{doc, oid::ObjectId, Document, to_bson};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateContactUsResult = Result<Option<models::Product>, Response>;

async fn _update_contact_us_form(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateContactUsResult {
    let products = db.products.find_one_and_update(filter, update, option).await.map_err(|e|{
        ResponseBuilder::query_error("products", e).into_response()
    })?;

    Ok(products)
}


pub async fn add_product_item(
    db: &DBExtension,
    product_id: &ObjectId,
    item: &models::ProductItem,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateContactUsResult {
    let filters = doc! {
        "_id": product_id
    };

    let update = doc! {
        "$push": {
            "items": to_bson(item).unwrap()
        }
    };

    _update_contact_us_form(db, filters, update, option).await
}
