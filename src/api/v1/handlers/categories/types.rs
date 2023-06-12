use crate::prelude::types::*;
use shoppa_core::parser::empty_string_as_none;

#[derive(Deserialize)]
pub struct GetCategoriesQueryParams {
    #[serde(deserialize_with = "empty_string_as_none")]
    pub parent: Option<ObjectId>,
}
