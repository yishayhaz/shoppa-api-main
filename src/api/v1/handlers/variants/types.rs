use crate::{prelude::types::*, db::models::{VariantType, VariantValue}};


#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantPayload {
    pub name: String,
    #[validate(length(min=2))]
    pub values: Vec<CreateVariantsValues>,
    #[serde(rename = "type")]
    pub type_: VariantType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantsValues {
    #[validate(length(min=1, max=15))]
    pub value: String,
    #[validate(length(min=1, max=15))]
    pub label: String,
}

impl Into<VariantValue> for CreateVariantsValues {
    fn into(self) -> VariantValue {
        VariantValue::new(self.value, self.label)
    }
}