use crate::prelude::types::*;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantPayload {
    pub name: String,
    #[validate(length(min=2))]
    pub values: Vec<String>
}
