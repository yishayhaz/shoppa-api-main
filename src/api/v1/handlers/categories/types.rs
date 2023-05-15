use crate::prelude::{types::*};


#[derive(Deserialize)]
pub struct CreateRootCatgoriePayload{
    pub name: String,
    pub variants: Option<Vec<ObjectId>>
}

#[derive(Deserialize)]
pub struct CreateInnerCatgoriePayload{
    pub name: String,
    pub variants: Option<Vec<ObjectId>>
}

#[derive(Deserialize)]
pub struct CreateInnerInnerCatgoriePayload{
    pub name: String,
    pub variants: Option<Vec<ObjectId>>
}

#[derive(Deserialize)]
pub struct GetCategoryInfo{
    pub category_ids: Vec<ObjectId>
}

#[derive(Deserialize)]
pub struct UpdateCategoryInfo{
    pub category_ids: Vec<ObjectId>
}
