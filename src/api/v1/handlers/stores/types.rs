use crate::prelude::types::*;
use shoppa_core::parser::empty_string_as_none;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchStoresQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>,
}
