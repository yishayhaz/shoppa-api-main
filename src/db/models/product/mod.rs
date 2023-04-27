mod fields;

use super::{
    common::{db_model, embedded_document, DBModel, EmbeddedDocument, RefrenceField},
    Categories, InnerCategories, InnerInnerCategories, Store, Variants,
};
use crate::prelude::{db_models::*, *};

// product model
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
    pub images: Vec<ProductImage>,
    pub items: Vec<ProductItem>, // pub product_info: Vec<String>
    pub analytics: ProductAnalytics,
}

//product item, eg variants represantion of the product
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductItem {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub price: f64,
    pub in_storage: u64,
    // this field describe the variant of the givem item: e.g: size L and color red.
    // so it will be uniqe with the product id to make sure there is no double items with
    // the same variants, the length of the variants field here need to be the same as the one in the parent product.
    pub variants: Vec<ItemVariants>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemVariants {
    // the variant _id field.
    pub variant_id: ObjectId,
    // the above variant value id in his values field.
    pub value_id: ObjectId,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAnalytics {
    pub views: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, EnumString)]
pub enum ProductSortBy {
    #[serde(alias = "popularity", alias = "pop", alias = "p", alias = "Popularity")]
    Popularity,
    #[serde(alias = "date", alias = "da", alias = "d", alias = "Date")]
    Date,
    #[serde(alias = "relevance", alias = "rel", alias = "r", alias = "Relevance")]
    Relevance
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
    ) -> Result<Self> {
        let store_id = match store.id() {
            Ok(id) => id,
            Err(_) => return Err(Error::Static("TODO")),
        };

        let categories = {
            let c_id = match categorie.id() {
                Ok(id) => id,
                Err(_) => return Err(Error::Static("TODO")),
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
            return Err(Error::Static("TODO"));
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
            images: vec![],
            items: vec![],
            analytics: ProductAnalytics { views: 0 },
        })
    }

    pub fn fields() -> &'static fields::ProductFields {
        &fields::FIELDS
    }

    pub fn add_item(
        &mut self,
        price: f64,
        in_storage: u64,
        new_item_variants: Vec<ItemVariants>,
    ) -> Result<&ProductItem> {
        let product_variants_length = match &self.variants {
            RefrenceField::NotPopulated(variants) => variants.len(),
            RefrenceField::Populated(variants) => variants.len(),
        };

        if new_item_variants.len() != product_variants_length {
            // not all variants are provided
            return Err(Error::Static("TODO"));
        }

        // if there are no variants, we can only have one item
        if product_variants_length == 0 {
            if self.items.len() != 0 {
                // already has the only variant possible
                return Err(Error::Static("TODO"));
            }
            let item = ProductItem::new(price, in_storage, vec![]);
            self.items.push(item);
            return Ok(self.items.last().unwrap());
        }

        for item in &self.items {
            if item.variants == new_item_variants {
                // variant already exists
                return Err(Error::Static("TODO"));
            }
        }

        let product_variants = match &self.variants {
            RefrenceField::NotPopulated(_) => {
                // to create a new item, we need to have the variants populated
                return Err(Error::Static("TODO"));
            }
            RefrenceField::Populated(v) => v,
        };

        for product_variant in product_variants {
            let mut found = false;
            // if the variant is populated we can assume that the id is populated
            let v_id = *product_variant.id().unwrap();
            for item_variant in &new_item_variants {
                // now we need to check if the provided variant value id exists in the variant
                if v_id == item_variant.variant_id {
                    found = product_variant
                        .values
                        .iter()
                        .find(|v| *v.id() == item_variant.value_id)
                        .is_some();
                }
            }
            if !found {
                // variant not found
                return Err(Error::Static("TODO"));
            }
        }

        let item = ProductItem::new(price, in_storage, new_item_variants);

        self.items.push(item);

        Ok(self.items.last().unwrap())
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

impl EmbeddedDocument for ProductItem {
    embedded_document!(ProductItem);
}

impl ProductItem {
    fn new(price: f64, in_storage: u64, variants: Vec<ItemVariants>) -> Self {
        Self {
            id: ObjectId::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            price,
            in_storage,
            variants,
        }
    }
}

impl PartialEq for ItemVariants {
    fn eq(&self, other: &Self) -> bool {
        self.variant_id == other.variant_id && self.value_id == other.value_id
    }
}
