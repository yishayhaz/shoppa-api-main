use crate::prelude::types::*;
use shoppa_core::{
    constans,
    parser::{deserialize_query_array, empty_string_as_none},
};
use validator::Validate;

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
