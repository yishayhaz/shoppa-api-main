use super::prelude::*;
use crate::{
    db::models::{VariantType, VariantValue, Variants},
    prelude::*,
};

type InsertVariantResult = Result<Variants>;

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
