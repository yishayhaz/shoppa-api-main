use super::{extract_insert_document_error, InsertDocumentErrors};
use crate::{
    db::models::{DBModel, Variants, VariantValue},
    helpers::types::DBExtension,
};

type InsertVariantResult = Result<Variants, InsertDocumentErrors>;

pub async fn new_variant(db: &DBExtension, name: String, value_names: Vec<String>) -> InsertVariantResult {

    let values = value_names
        .into_iter()
        .map(|name| VariantValue::new(name))
        .collect();

    let mut variant = Variants::new(name, values);

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
