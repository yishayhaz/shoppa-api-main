use super::types::UserRegisterPayload;
use crate::{
    api::v1::middlewares::*,
    db::{inserts, inserts::InsertDocumentErrors, queries, updates},
    helpers::{cookies::set_access_cookie, security},
    prelude::{handlers::*, *},
};

pub async fn signup(
    db: DBExtension,
    cookies: Cookies,
    Level1AccessOrNone(token_data): Level1AccessOrNone,
    JsonWithValidation(payload): JsonWithValidation<UserRegisterPayload>,
) -> HandlerResult {
    // checking if email alredy in use
    let user_exists = queries::get_user_by_email(&db, &payload.email).await?;

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
        let user = updates::update_user_to_level_2(
            &db,
            &token_data.unwrap().user_id,
            &payload.email,
            &password,
            &payload.name,
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

    let user = match inserts::new_level_2_user(&db, payload.email, password, payload.name).await {
        Ok(v) => v,
        Err(e) => match e {
            InsertDocumentErrors::UnknownError => {
                return Ok(ResponseBuilder::<u16>::error("", None, None, None).into_response());
            }
            _ => return Ok(e.into_response()),
        },
    };

    set_access_cookie(&cookies, &user)?;

    Ok(ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response())
}

pub async fn signup_level_1() {}
