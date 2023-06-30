use crate::{
    api::stores::middlewares::CurrentUser,
    db::{AxumDBExtansion, StoreUserFunctionsForStoreUser},
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::ResponseBuilder;

pub async fn get_me(db: AxumDBExtansion, current_user: CurrentUser) -> HandlerResult {
    let user = db.get_me(&current_user.user_id).await?;

    Ok(ResponseBuilder::success(user, None, None).into_response())
}
