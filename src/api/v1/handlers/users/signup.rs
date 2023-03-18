use super::types::UserRegisterPayload;
use crate::helpers::json::JsonWithValidation;

pub async fn signup(JsonWithValidation(_payload): JsonWithValidation<UserRegisterPayload>) {}

pub async fn signup_level_1() {}
