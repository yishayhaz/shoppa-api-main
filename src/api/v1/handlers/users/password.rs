use super::types::ChangePasswordPayload;
use crate::{
    api::v1::middlewares::*,
    db::{AxumDBExtansion, UserFunctions},
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, security, ResponseBuilder};

pub async fn change_password(
    db: AxumDBExtansion,
    Level2Access(token_data): Level2Access,
    JsonWithValidation(payload): JsonWithValidation<ChangePasswordPayload>,
) -> HandlerResult {
    let user = match db.get_user_by_id(&token_data.user_id, None, None, None).await? {
        Some(v) => v,
        None => {
            return Ok(
                ResponseBuilder::<u16>::error("", None, Some("User not found"), None)
                    .into_response(),
            );
        }
    };

    // If the user is level 2 he must have a password in the db, and this route allows only level two and above
    if !security::verify_password(&payload.old_password, &user.password.unwrap())? {
        return Ok(ResponseBuilder::<u16>::error(
            "",
            None,
            Some("Old password doesnt match"),
            None,
        )
        .into_response());
    };

    let new_password_hashed = security::hash_password(&payload.new_password)?;

    db.update_user_password(&token_data.user_id, &new_password_hashed, None)
        .await?;

    Ok(ResponseBuilder::success(Some(""), None, None).into_response())
}
