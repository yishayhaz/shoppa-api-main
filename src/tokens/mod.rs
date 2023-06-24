use crate::helpers::env::ENV_VARS;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shoppa_core::{
    db::models::{DBModel, StoreUser},
    random::random_string,
    security::TokenManager,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreUserTokenData {
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreUserRegistrationTokenData {
    pub user_id: ObjectId,
    pub secret: String,
}

lazy_static! {
    pub static ref STORE_USER_TOKEN_MANAGER: TokenManager<StoreUserTokenData> = TokenManager::new(
        "store-api",
        ENV_VARS.STORE_USER_LOGIN_TOKEN_SECRET.as_str(),
        90
    );
    pub static ref STORE_USER_REGISTRATION_TOKEN_MANAGER: TokenManager<StoreUserRegistrationTokenData> =
        TokenManager::new(
            "store-api",
            ENV_VARS.STORE_USER_REGISTRATION_TOKEN_SECRET.as_str(),
            14
        );
}

impl StoreUserTokenData {
    pub fn new(user_id: ObjectId) -> Self {
        Self { user_id }
    }
}

impl StoreUserRegistrationTokenData {
    pub fn new(user_id: ObjectId) -> Self {
        Self {
            user_id,
            secret: random_string(64),
        }
    }
}

impl Into<StoreUserTokenData> for &StoreUser {
    fn into(self) -> StoreUserTokenData {
        StoreUserTokenData::new(self.id().unwrap().clone())
    }
}

impl Into<StoreUserRegistrationTokenData> for &StoreUser {
    fn into(self) -> StoreUserRegistrationTokenData {
        StoreUserRegistrationTokenData::new(self.id().unwrap().clone())
    }
}
