use super::prelude::*;
use crate::{
    db::models::{VariantType, VariantValue, Variants},
    prelude::*,
};

type InsertVariantResult = Result<Variants>;
type InsertVariantValueResult = Result<VariantValue>;

pub async fn new_variant(
    db: &DBExtension,
    name: String,
    values: Vec<impl Into<VariantValue>>,
    type_: VariantType,
) -> InsertVariantResult {
    let values = values.into_iter().map(|item| item.into()).collect();

    let mut variant = Variants::new(name, values, type_);

    let res = db
        .variants
        .insert_one(&variant, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    variant.update_id(id);

    Ok(variant)
}

pub async fn add_variant_value(db: &DBExtension,
    variant_id: &ObjectId,
    label: &String,
    value: &String,
  ) -> InsertVariantValueResult {
    todo!("add_variant_value");
  }