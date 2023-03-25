use super::prelude::*;
use crate::db::models::{Categories, InnerCategories, };//InnerInnerCategories};

type InsertCategoriesResult = Result<Categories, InsertDocumentErrors>;
type InsertInnerCategoriesResult = Result<InnerCategories, InsertDocumentErrors>;
// type InsertInnerInnerCategoriesResult = Result<InnerInnerCategories, InsertDocumentErrors>;

pub async fn new_root_catagorie(db: &DBExtension, name: String) -> InsertCategoriesResult {
    let mut catagorie = Categories::new(name, vec![]);

    let res = match db.categories.insert_one(&catagorie, None).await {
        Ok(v) => v,
        Err(err) => return Err(extract_insert_document_error(*err.kind)),
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    catagorie.update_id(id);

    Ok(catagorie)
}

pub async fn new_inner_catagorie(
    db: &DBExtension,
    name: String,
    categorie_id: &ObjectId,
) -> InsertInnerCategoriesResult {
    let inner_catagorie = InnerCategories::new(name, vec![]);

    let catgories_fields = Categories::fields();

    let inner_bson = match inner_catagorie.into_bson(){
        Ok(v) => v,
        Err(_) => return Err(InsertDocumentErrors::BsonConversionError)
    };

    let _ = db
        .categories
        .update_one(
            doc! {"_id": categorie_id},
            doc! {
                "$push": {
                    catgories_fields.categories: inner_bson
                }
            },
            None,
        )
        .await;

    Ok(inner_catagorie)
}

// pub async fn new_inner_inner_catagorie(
//     db: &DBExtension,
//     name: String,
//     categorie_id: &ObjectId,
//     inner_categorie_id: &ObjectId,
// ) -> InsertInnerInnerCategoriesResult {
//     let inner_inner_catagorie = InnerInnerCategories::new(name);

//     Ok(inner_inner_catagorie)
// }
