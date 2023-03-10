use super::types::UserLoginPayload;
use crate::helpers::json::JsonWithValidation;

pub async fn login(
    JsonWithValidation(payload): JsonWithValidation<UserLoginPayload>
) -> String {

    payload.email
}

pub async fn logout(

) {

}