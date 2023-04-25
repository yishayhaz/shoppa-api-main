use super::types::ChangePasswordPayload;
use crate::{
    db::{queries, updates},
    helpers::security,
    prelude::{handlers::*},
    api::v1::middlewares::*,
};

pub async fn change_password(
    db: DBExtension,
    Level2Access(token_data): Level2Access,
    JsonWithValidation(payload): JsonWithValidation<ChangePasswordPayload>,
) -> HandlerResponse {
    let user = match queries::get_user_by_id(&db, &token_data.user_id).await? {
        Some(v) => v,
        None => {
            return Err(
                ResponseBuilder::<u16>::error("", None, Some("User not found"), None)
                    .into_response(),
            );
        }
    };

    // If the user is level 2 he must have a password in the db, and this route allows only level two and above
    if !security::verify_password(&payload.old_password, &user.password.unwrap())? {
        return Err(ResponseBuilder::<u16>::error(
            "",
            None,
            Some("Old password doesnt match"),
            None,
        )
        .into_response());
    };

    let new_password_hashed = security::hash_password(&payload.new_password)?;

    let _ = updates::update_user_password(&db, &token_data.user_id, &new_password_hashed).await?;

    Ok(ResponseBuilder::success(Some(""), None, None).into_response())
}
