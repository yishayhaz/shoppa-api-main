use super::prelude::*;
use crate::{db::models::{Categories, InnerCategories, InnerInnerCategories}, prelude::*};

type InsertCategoriesResult = Result<Categories>;
type InsertInnerCategoriesResult = Result<InnerCategories>;
type InsertInnerInnerCategoriesResult = Result<InnerInnerCategories>;

pub async fn new_root_catagorie(
    db: &DBExtension,
    name: String,
    variants_ids: Option<Vec<ObjectId>>,
) -> InsertCategoriesResult {
    let mut catagorie = Categories::new(name, vec![], variants_ids);

    let res = db.categories.insert_one(&catagorie, None).await.map_err(|e| {
        Error::DBError(("categories", e))
    })?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    catagorie.update_id(id);

    Ok(catagorie)
}

pub async fn new_inner_catagorie(
    db: &DBExtension,
    name: String,
    categorie_id: &ObjectId,
    variants_ids: Option<Vec<ObjectId>>,
) -> InsertInnerCategoriesResult {
    let inner_catagorie = InnerCategories::new(name, vec![], variants_ids);

    let catgories_fields = Categories::fields();

    let inner_bson = inner_catagorie.into_bson()?;
    // TODO handle error
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

pub async fn new_inner_inner_catagorie(
    db: &DBExtension,
    name: String,
    categorie_id: &ObjectId,
    inner_categorie_id: &ObjectId,
    variants_ids: Option<Vec<ObjectId>>,
) -> InsertInnerInnerCategoriesResult {
    let inner_inner_catagorie = InnerInnerCategories::new(name, variants_ids);

    let catgories_fields = Categories::fields();

    let inner_bson = inner_inner_catagorie.into_bson()?;
    // TODO handle error
    let _ = db
        .categories
        .update_one(
            doc! {
                "_id": categorie_id,
                format!("{}.{}", catgories_fields.categories, catgories_fields.categories().id): inner_categorie_id
            },
            doc! {
                "$push": {
                    format!("{}.$.{}", catgories_fields.categories, catgories_fields.categories().categories): inner_bson
                }
            },
            None,
        )
        .await;

    Ok(inner_inner_catagorie)
}
