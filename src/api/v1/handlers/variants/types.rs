use crate::{prelude::types::*, db::models::VariantType};


#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantPayload {
    pub name: String,
    #[validate(length(min=2))]
    pub values: Vec<String>,
    #[serde(rename = "type")]
    pub type_: VariantType,
}
