use crate::{
    db::models::{VariantType, VariantValue},
    helpers::parser::deserialize_query_array,
    prelude::types::*,
};

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantPayload {
    pub name: String,
    #[validate(length(min = 2))]
    pub values: Vec<CreateVariantsValues>,
    #[serde(rename = "type")]
    pub type_: VariantType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantsValues {
    #[validate(length(min = 1, max = 15))]
    pub value: String,
    #[validate(length(min = 1, max = 15))]
    pub label: String,
}

impl Into<VariantValue> for CreateVariantsValues {
    fn into(self) -> VariantValue {
        VariantValue::new(self.value, self.label)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct UpdateVariantBasicInfoPayload {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<VariantType>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantValuePayload {
    pub value: String,
    pub label: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct UpdateVariantValuePayload {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct GetVariantsByIdsQuery {
    #[serde(deserialize_with = "deserialize_query_array")]
    pub variants_ids: Vec<ObjectId>,
}
