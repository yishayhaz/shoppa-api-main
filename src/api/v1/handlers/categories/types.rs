use crate::prelude::{types::*};


#[derive(Deserialize)]
pub struct GetCategoryInfo {
    pub category_ids: Vec<ObjectId>
}

#[derive(Deserialize)]
pub struct DeleteCategory {
    pub category_ids: Vec<ObjectId>
}

