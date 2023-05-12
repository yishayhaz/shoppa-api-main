use super::prelude::*;
use crate::{
    db::models::{Categories, InnerCategories, InnerInnerCategories, Product, Store},
    prelude::*,
};

type InsertProductResult = Result<Product>;

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
    let mut product = Product::new(
        store,
        brand,
        description,
        keywords,
        name,
        categorie,
        inner_categorie,
        inner_inner_categorie,
        variants,
    )?;

    let res = db.products.insert_one(&product, None).await.map_err(|e| {
        Error::DBError(("products", e))
    })?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    product.update_id(id);

    Ok(product)
}
