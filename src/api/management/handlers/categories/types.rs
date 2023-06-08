use crate::prelude::{types::*};


#[derive(Deserialize)]
pub struct CreateCatgoryPayload{
    pub name: String,
    pub variants: Option<Vec<ObjectId>>,
    pub parent: Option<ObjectId>
}


#[derive(Deserialize)]
pub struct GetCategoryInfo {
    pub category_ids: Vec<ObjectId>
}

#[derive(Deserialize)]
pub struct DeleteCategory {
    pub category_ids: Vec<ObjectId>
}

#[derive(Deserialize)]
pub struct UpdateCategoryInfo {
    pub category_ids: Vec<ObjectId>,
    pub name: Option<String>,
    pub variants: Option<Vec<ObjectId>>,
}
