use super::types::UserRegisterPayload;
use crate::{
    db::{inserts, inserts::InsertDocumentErrors},
    helpers::{
        cookies::set_access_cookie,
        json::JsonWithValidation,
        security,
        types::{DBExtension, HandlerResponse, ResponseBuilder},
    },
};
use axum::response::IntoResponse;
use tower_cookies::Cookies;

// TODO if the user has a token update him insted of creating a new user
pub async fn signup(
    db: DBExtension,
    cookies: Cookies,
    JsonWithValidation(payload): JsonWithValidation<UserRegisterPayload>,
) -> HandlerResponse {
    let password = security::hash_password(&payload.password)?;

    let user = match inserts::new_level_2_user(&db, payload.email, password, payload.name).await {
        Ok(v) => v,
        Err(e) => match e {
            InsertDocumentErrors::UnknownError => {
                return Err(ResponseBuilder::<u16>::error("", None, None, None).into_response());
            }
            InsertDocumentErrors::AlredyExists => {
                return Err(ResponseBuilder::<u16>::error(
                    "",
                    None,
                    Some("looks like you alredy subscribed"),
                    Some(409),
                )
                .into_response());
            }
        },
    };

    let user_get_me = user.to_get_me()?;

    set_access_cookie(&cookies, &user)?;

    Ok(ResponseBuilder::success(Some(user_get_me), None, None).into_response())
}

pub async fn signup_level_1() {}
