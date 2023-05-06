use crate::prelude::types::*;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateStorePayload {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchStoresQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub free_text: Option<String>
}