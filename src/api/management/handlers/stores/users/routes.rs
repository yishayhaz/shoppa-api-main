use super::types;
use crate::{
    db::AxumDBExtansion,
    emails::AdminEmailFunctions,
    helpers::types::AxumEmailClientExtension,
    prelude::*,
    tokens::{StoreUserRegistrationTokenData, STORE_USER_REGISTRATION_TOKEN_MANAGER},
};
use axum::response::IntoResponse;
use bson::doc;
use shoppa_core::{
    db::models::{DBModel, StoreUser},
    extractors::JsonWithValidation,
    ResponseBuilder,
};

pub async fn create_store_user(
    db: AxumDBExtansion,
    email_client: AxumEmailClientExtension,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStoreUserPayload>,
) -> HandlerResult {
    let store = db.get_store_by_id(&payload.store, None, None, None).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<()>::error("Store not found", None, None, Some(404)).into_response(),
        );
    }

    let store = store.unwrap();

    let store_user = db.insert_new_store_user(payload, None, None).await?;

    let store_user_ref = &store_user;

    let token_data: StoreUserRegistrationTokenData = store_user_ref.into();

    db.update_store_user_by_id(
        store_user.id().unwrap(),
        doc! {
            "$set": {
                StoreUser::fields().registration_token_secret: &token_data.secret
            }
        },
        None,
        None,
    )
    .await?;

    let token = STORE_USER_REGISTRATION_TOKEN_MANAGER.generate_urlsafe_token(token_data, None)?;

    let email = email_client
        .new_store_user_email(
            format!("https://shoppa.co.il?token={}", token),
            store_user.name.clone(),
            store.logo.map(|l| l.path).unwrap_or_default(),
            store.name,
        )
        .add_to((store_user.email.clone(), store_user.name.clone()).into())
        .build();

    let _ = email_client.send(email).await;

    Ok(ResponseBuilder::success(Some(store_user), None, None).into_response())
}
