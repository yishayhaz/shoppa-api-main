use crate::{db::models, helpers::types::DBExtension, prelude::*};
use bson::{doc, oid::ObjectId, to_bson, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateContactUsResult = Result<Option<models::Product>>;

async fn update_product(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateContactUsResult {
    let products = db
        .products
        .find_one_and_update(filter, update, option)
        .await
        .map_err(|e| Error::DBError(("product", e)))?;

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

    update_product(db, filters, update, option).await
}
