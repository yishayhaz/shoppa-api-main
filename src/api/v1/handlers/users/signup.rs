use super::types::UserRegisterPayload;
use crate::{
    db::{inserts, inserts::InsertDocumentErrors},
    helpers::{
        json::JsonWithValidation,
        security::hash_password,
        types::{DBExtension, HandlerResponse, ResponseBuilder},
    },
};
use axum::response::IntoResponse;

// TODO if the user has a token update him insted of creating a new user
pub async fn signup(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<UserRegisterPayload>,
) -> HandlerResponse {
    let password = hash_password(&payload.password)?;

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

    Ok(ResponseBuilder::success(Some(""), None, None).into_response())
}

pub async fn signup_level_1() {}
