use super::super::prelude::types::*;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateProductPayload {
    #[validate(length(min = 8, max = 64))]
    pub name: String,
    // 3 categories must be provided
    #[validate(length(min = 3, max = 3))]
    pub catagories: Vec<ObjectId>,

    pub variants: Vec<ObjectId>,
    pub store: ObjectId,
    pub keywords: Vec<String>,
    pub brand: Option<String>,
}
