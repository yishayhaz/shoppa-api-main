use crate::prelude::types::*;
use shoppa_core::{
    constans,
    db::models::{VariantType, VariantValue, Variants},
    parser::{deserialize_query_array, empty_string_as_none},
};
use validator::Validate;

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
    pub new_values: Option<Vec<CreateVariantsValues>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateVariantValuePayload {
    pub value: String,
    pub label: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateVariantValuePayload {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct GetVariantsByIdsQuery {
    #[serde(deserialize_with = "deserialize_query_array")]
    #[validate(length(min = 1, max = "constans::PRODUCT_MAX_VARIANTS"))]
    pub variants_ids: Vec<ObjectId>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct GetVariantsAutocompleteQuery {
    #[serde(default, deserialize_with = "deserialize_query_array")]
    pub categories_ids: Vec<ObjectId>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
}


impl Into<Variants> for CreateVariantPayload {
    fn into(self) -> Variants {
        Variants::new(
            self.name,
            self.values.into_iter().map(|v| v.into()).collect(),
            self.type_,
        )
    }
}

impl Validate for UpdateVariantValuePayload {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.value.is_none() && self.label.is_none() {
            errors.add(
                "value",
                ValidationError::new("value or label must be provided"),
            );
            errors.add(
                "label",
                ValidationError::new("value or label must be provided"),
            );
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}
