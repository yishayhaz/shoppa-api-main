use crate::prelude::types::*;
use shoppa_core::{db::models::StoreUser, validators::phone_number_validator};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateStoreUserPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "phone_number_validator")]
    pub phone_number: Option<String>,
    pub store: ObjectId,
    pub name: String,
}

impl Into<StoreUser> for CreateStoreUserPayload {
    fn into(self) -> StoreUser {
        StoreUser::new(
            self.store,
            self.name,
            self.email,
            self.phone_number,
            String::new(),
        )
    }
}
