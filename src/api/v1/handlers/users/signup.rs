use super::types::UserRegisterPayload;
use crate::{
    api::v1::middlewares::*,
    db::{AxumDBExtansion, UserFunctions},
    helpers::cookies::set_access_cookie,
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, security, ResponseBuilder};
use tower_cookies::Cookies;

pub async fn signup(
    db: AxumDBExtansion,
    cookies: Cookies,
    Level1AccessOrNone(token_data): Level1AccessOrNone,
    JsonWithValidation(payload): JsonWithValidation<UserRegisterPayload>,
) -> HandlerResult {
    // checking if email alredy in use
    let user_exists = db.get_user_by_email(&payload.email, None, None).await?;

    if user_exists.is_some() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("User alredy exists"), Some(409))
                .into_response(),
        );
    }

    // hashing password
    let password = security::hash_password(&payload.password)?;

    // if there is some token data we upgrade the user to level 2
    if token_data.is_some() {
        let user = db
            .update_user_to_level_2(
                &token_data.unwrap().user_id,
                &payload.email,
                &password,
                &payload.name,
                None,
            )
            .await?;

        match user {
            // If we managed to create a user we set cookie and return res
            Some(user) => {
                set_access_cookie(&cookies, &user)?;

                return Ok(
                    ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response(),
                );
            }
            // else we try to create a completly new user
            None => {}
        }
    }
    todo!("signup new level 2")
    // let user = inserts::new_level_2_user(&db, payload.email, password, payload.name).await?;

    // set_access_cookie(&cookies, &user)?;

    // Ok(ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response())
}

pub async fn signup_level_1() {}
