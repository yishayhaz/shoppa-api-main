use super::prelude::*;
use crate::db::models::{Categories, InnerCategories, InnerInnerCategories, Product, Store};

type InsertProductResult = Result<Product, InsertDocumentErrors>;

pub async fn new_product(
    db: &DBExtension,
    store: &Store,
    brand: Option<String>,
    description: String,
    keywords: Vec<String>,
    name: String,
    categorie: &Categories,
    inner_categorie: &InnerCategories,
    inner_inner_categorie: &InnerInnerCategories,
    variants: Vec<ObjectId>,
) -> InsertProductResult {
    let product = Product::new(
        store,
        brand,
        description,
        keywords,
        name,
        categorie,
        inner_categorie,
        inner_inner_categorie,
        variants,
    );

    if product.is_err() {
        return Err(InsertDocumentErrors::InvalidArgumentsForModel);
    }

    let mut product = product.unwrap();

    let res = match db.products.insert_one(&product, None).await {
        Ok(v) => v,
        Err(err) => return Err(extract_insert_document_error(*err.kind)),
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    product.update_id(id);

    Ok(product)
}
