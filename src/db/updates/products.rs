use crate::{
    db::models::{self, EmbeddedDocument, FileDocument},
    helpers::types::DBExtension,
    prelude::*,
};
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

pub async fn add_image_to_product(
    db: &DBExtension,
    product_id: &ObjectId,
    image: FileDocument,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateProductResult {
    let filters = doc! {
        "_id": product_id
    };

    let update = doc! {
        "$push": {
            "images": image.into_bson()?
        }
    };

    update_product(db, filters, update, option).await
}

pub async fn edit_product_item(
    db: &DBExtension,
    product_id: &ObjectId,
    item_id: &ObjectId,
    price: Option<f64>,
    in_storage: Option<u64>,
    name: Option<String>,
    images_refs: Option<Vec<ObjectId>>,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateProductResult {
    let filters = doc! {
        "_id": product_id,
        "items._id": item_id
    };

    let mut update = doc! {};

    if let Some(price) = price {
        update.insert("items.$.price", price);
    }

    if let Some(in_storage) = in_storage {
        update.insert("items.$.in_storage", in_storage as i64);
    }

    if let Some(name) = name {
        if name == "" {
            update.insert::<_, Option<String>>("items.$.name", None);
        } else {
            update.insert("items.$.name", name);
        }
    }

    if let Some(images_refs) = images_refs {
        update.insert("items.$.images_refs", images_refs);
    }

    update.insert("items.$.updated_at", chrono::Utc::now());

    let update = doc! {
        "$set": update
    };

    update_product(db, filters, update, option).await
}
