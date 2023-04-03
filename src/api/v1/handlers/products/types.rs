use super::super::prelude::types::*;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateProductPayload {
    #[validate(length(min = 8, max = 64))]
    pub name: String,
    // 3 categories must be provided
    #[validate(length(min = 3, max = 3))]
    pub catagories: Vec<ObjectId>,

    pub variants: Option<Vec<ObjectId>>,
    pub store: ObjectId,
    pub keywords: Option<Vec<String>>,
    pub brand: Option<String>,
    #[validate(length(min = 8))]
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct GetProductQueryParams {
    #[serde(deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub store_id: Option<ObjectId>
}