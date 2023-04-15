mod fields;

use super::{
    common::{db_model, DBModel, EmbeddedDocument, RefrenceField},
    prelude::*,
    Categories, InnerCategories, InnerInnerCategories, Store, Variants,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub brand: Option<String>,
    // between 8 latters to 64
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub store: RefrenceField<Store, StoreField>,
    // Not likely that it will be populated.
    // But if it will be, I need to make it only contain the
    // Caregories in the Vec<CategoriesField>>
    pub categories: RefrenceField<Categories, Vec<CategoriesField>>,
    pub variants: RefrenceField<Vec<Variants>, Vec<ObjectId>>,
    pub images: Vec<ProductImage>
    // pub product_info: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoriesField {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreField {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductImage {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl DBModel for Product {
    fn get_collection_name() -> &'static str {
        "products"
    }

    fn get_indexes() -> Vec<IndexModel> {
        // we can add language key for the product to imrpove the text index,
        // or to set the default to hebrew

        let text_index_options = IndexOptions::builder()
            .weights(doc! {
                "name": 200,
                "keywords": 100,
                "description": 100,
                "brand": 50,
                "categories.name": 50,
                "store.name": 20
            })
            .name(String::from("search_text_index"))
            .default_language(String::from("none"))
            .build();

        let text_index = IndexModel::builder()
            .keys(doc! {
                "name": "text",
                "description": "text",
                "categories.name": "text",
                "brand": "text",
                "keywords": "text",
                "store.name": "text"
            })
            .options(text_index_options)
            .build();

        let unique_index_options = IndexOptions::builder()
            .unique(true)
            .name(String::from("unique_product_for_store"))
            .build();

        let uniqe_index = IndexModel::builder()
            .keys(doc! {
                "name": 1,
                "store._id": 1
            })
            .options(unique_index_options)
            .build();

        vec![text_index, uniqe_index]
    }

    db_model!(Product);
}

impl Product {
    pub fn new(
        store: &Store,
        brand: Option<String>,
        description: String,
        keywords: Vec<String>,
        name: String,
        categorie: &Categories,
        inner_categorie: &InnerCategories,
        inner_inner_categorie: &InnerInnerCategories,
        variants: Vec<ObjectId>,
    ) -> Result<Self, ()> {
        let store_id = match store.id() {
            Ok(id) => id,
            Err(_) => return Err(()),
        };

        let categories = {
            let c_id = match categorie.id() {
                Ok(id) => id,
                Err(_) => return Err(()),
            };

            vec![
                CategoriesField::new(c_id.clone(), categorie.name.clone()),
                CategoriesField::new(inner_categorie.id().clone(), inner_categorie.name.clone()),
                CategoriesField::new(
                    inner_inner_categorie.id().clone(),
                    inner_inner_categorie.name.clone(),
                ),
            ]
        };

        let mut allowed_variants = Vec::new();

        match &categorie.allowed_variants {
            RefrenceField::Populated(var) => {
                for v in var {
                    if let Ok(id) = v.id() {
                        allowed_variants.push(id);
                    }
                }
            }
            RefrenceField::NotPopulated(var) => {
                allowed_variants.extend(var);
            }
        };

        match &inner_categorie.allowed_variants {
            RefrenceField::Populated(var) => {
                for v in var {
                    if let Ok(id) = v.id() {
                        allowed_variants.push(id);
                    }
                }
            }
            RefrenceField::NotPopulated(var) => {
                allowed_variants.extend(var);
            }
        };

        match &inner_inner_categorie.allowed_variants {
            RefrenceField::Populated(var) => {
                for v in var {
                    if let Ok(id) = v.id() {
                        allowed_variants.push(id);
                    }
                }
            }
            RefrenceField::NotPopulated(var) => {
                allowed_variants.extend(var);
            }
        };

        if !variants.iter().all(|v| allowed_variants.contains(&v)) {
            return Err(());
        };

        Ok(Self {
            id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            brand,
            name,
            description,
            keywords,
            store: RefrenceField::NotPopulated(StoreField::new(
                store_id.clone(),
                store.name.clone(),
            )),
            categories: RefrenceField::NotPopulated(categories),
            variants: RefrenceField::NotPopulated(variants),
            images: vec![]
        })
    }

    pub fn fields() -> &'static fields::ProductFields {
        &fields::FIELDS
    }
}

impl StoreField {
    fn new(id: ObjectId, name: String) -> Self {
        Self { id, name }
    }
}

impl CategoriesField {
    fn new(id: ObjectId, name: String) -> Self {
        Self { id, name }
    }
}
