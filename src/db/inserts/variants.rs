use super::prelude::*;
use crate::db::models::{VariantValue, Variants, VariantType};

type InsertVariantResult = Result<Variants, InsertDocumentErrors>;

pub async fn new_variant(
    db: &DBExtension,
    name: String,
    values: Vec<impl Into<VariantValue>>,
    type_: VariantType,
) -> InsertVariantResult {
    let values = values
        .into_iter()
        .map(|item| item.into())
        .collect();

    let mut variant = Variants::new(name, values, type_);

    let res = match db.variants.insert_one(&variant, None).await {
        Ok(v) => v,
        Err(err) => return Err(extract_insert_document_error(*err.kind)),
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    variant.update_id(id);

    Ok(variant)
}
