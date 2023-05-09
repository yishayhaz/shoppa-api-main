use crate::{db::models::{self, EmbeddedDocument}, helpers::types::DBExtension, prelude::*};
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateProductResult = Result<Option<models::Product>>;

async fn update_product(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateProductResult {
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
) -> UpdateProductResult {
    let filters = doc! {
        "_id": product_id
    };

    let update = doc! {
        "$push": {
            "items": item.into_bson()?
        }
    };

    update_product(db, filters, update, option).await
}


pub async fn add_view_to_product(
    db: &DBExtension,
    product_id: &ObjectId,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateProductResult {
    let filters = doc! {
        "_id": product_id
    };

    let update = doc! {
        "$inc": {
            "analytics.views": 1
        }
    };

    update_product(db, filters, update, option).await
}
