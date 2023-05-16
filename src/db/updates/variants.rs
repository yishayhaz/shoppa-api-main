use crate::{db::models, helpers::types::DBExtension, prelude::*};
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateVariantResult = Result<Option<models::Variants>>;

pub async fn update_variant_basic_info(db: &DBExtension,
    variant_id: &ObjectId,
    name: &Option<String>,
    type_: &Option<models::VariantType>) -> UpdateVariantResult {
    let filter = doc! {
        "_id": variant_id
    };

    let mut update = doc! {};

    if let Some(name) = name {
        update.insert(models::Variants::fields().name, name);
    }

    // TODO: omer make it work!
    // if let Some(type_) = type_ {
    //     update.insert("type", type_);
    // }

    let update = doc! {
        "$set": update
    };

    let variant = db
        .variants
        .find_one_and_update(filter, update, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    Ok(variant)

}

pub async fn update_variant_value(db: &DBExtension,
  variant_id: &ObjectId,
  value_id: &ObjectId,
  label: &Option<String>,
  value: &Option<String>,
) -> UpdateVariantResult {
  todo!("update_variant_value");
}