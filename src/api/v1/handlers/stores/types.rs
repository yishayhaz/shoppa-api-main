use crate::prelude::types::*;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateStorePayload {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub description: String,
    pub location: String,
}