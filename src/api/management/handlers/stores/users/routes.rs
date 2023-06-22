use super::types;
use crate::{
    db::AxumDBExtansion, emails::AdminEmailFunctions, helpers::types::AxumEmailClientExtension,
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, ResponseBuilder};

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

    let email = email_client
        .new_store_user_email(
            "https://shoppa.co.il".to_string(),
            store_user.name.clone(),
            store.logo.map(|l| l.path).unwrap_or_default(),
            store.name,
        )
        .add_cc((store_user.email.clone(), store_user.name.clone()).into())
        .build();

    match email_client.send(email).await {
        Ok(r) => {
            tracing::info!("Email sent: {:?}", r);
        }
        Err(e) => {
            tracing::error!("Failed to send email: {:?}", e);
        }
    }
    // TODO send email to user and generate registration link

    Ok(ResponseBuilder::success(Some(store_user), None, None).into_response())
}
