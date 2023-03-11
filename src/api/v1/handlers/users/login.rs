use super::types::UserLoginPayload;
use crate::helpers::{json::JsonWithValidation, types::{ResponseBuilder, DBExtension, HandlerResponse}, security};
use crate::db::queries;
use axum::response::IntoResponse;


pub async fn login(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<UserLoginPayload>
) -> HandlerResponse {

    let user = queries::get_user_by_email(&db, payload.email).await?;

    let user_not_found = ResponseBuilder::<u16>::error(
        None,
        Some(String::from("User not found")),
        Some(404),
    ).into_response();

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(user_not_found)
        }
    };

    let user_password = match &user.password {
        Some(user_password) => user_password,
        None => {
            return Err(user_not_found)
        }
    };

    if !security::verify_password(&payload.password, user_password)?{
        return Err(user_not_found)
    }

    Ok(ResponseBuilder::success(Some(user), None, None).into_response())
}

pub async fn logout(

) -> HandlerResponse {

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}