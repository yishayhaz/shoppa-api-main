use crate::helpers::env::ENV_VARS;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shoppa_core::{
    db::models::{DBModel, RefrenceField, StoreUser, User, UserStatus, CheckOutSession},
    random::random_string,
    security::TokenManager,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreUserTokenData {
    pub user_id: ObjectId,
    #[serde(rename = "secret")]
    pub token_secret: String,
    pub store_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreUserRegistrationTokenData {
    pub user_id: ObjectId,
    pub name: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenData {
    pub user_id: ObjectId,
    pub secret: String,
    pub guest: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckOutSessionTokenData {
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
    pub static ref USER_TOKEN_MANAGER: TokenManager<UserTokenData> =
        TokenManager::new("store-api", ENV_VARS.LOGIN_TOKEN_SECRET.as_str(), 90);
    pub static ref CHECKOUT_SESSION_TOKEN_MANAGER: TokenManager<CheckOutSessionTokenData> =
        TokenManager::new(
            "store-api",
            ENV_VARS.CHECKOUT_SESSION_TOKEN_SECRET.as_str(),
            1
        );
}

impl StoreUserTokenData {
    pub fn new(user_id: ObjectId, store_id: ObjectId) -> Self {
        Self {
            user_id,
            token_secret: random_string(32),
            store_id,
        }
    }
}

impl StoreUserRegistrationTokenData {
    pub fn new(user_id: ObjectId, name: String) -> Self {
        Self {
            user_id,
            name,
            secret: random_string(64),
        }
    }
}

impl UserTokenData {
    pub fn new(user_id: ObjectId, guest: bool) -> Self {
        Self {
            user_id,
            secret: random_string(32),
            guest,
        }
    }
}

impl Into<StoreUserTokenData> for &StoreUser {
    fn into(self) -> StoreUserTokenData {
        let store_id = match &self.store {
            RefrenceField::Populated(store) => store.id().unwrap().clone(),
            RefrenceField::NotPopulated(store_id) => store_id.clone(),
        };
        StoreUserTokenData::new(self.id().unwrap().clone(), store_id)
    }
}

impl Into<StoreUserRegistrationTokenData> for &StoreUser {
    fn into(self) -> StoreUserRegistrationTokenData {
        StoreUserRegistrationTokenData::new(self.id().unwrap().clone(), self.name.clone())
    }
}

impl Into<UserTokenData> for &User {
    fn into(self) -> UserTokenData {
        UserTokenData::new(self.id().unwrap().clone(), self.status == UserStatus::Guest)
    }
}

impl Into<CheckOutSessionTokenData> for &CheckOutSession {
    fn into(self) -> CheckOutSessionTokenData {
        CheckOutSessionTokenData {
            secret: self.secret.clone(),
        }
    }
}
