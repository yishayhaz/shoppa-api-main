use super::types::AddUserAddress;
use crate::api::v1::middlewares::CurrentUser;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;

pub async fn get_user_addresses(db: AxumDBExtansion, current_user: CurrentUser) -> HandlerResult {
    let user = db
        .get_user_by_id(&current_user.user_id, None, None, None)
        .await?;

    todo!()
}

pub async fn add_user_address(db: AxumDBExtansion, current_user: CurrentUser) -> HandlerResult {
    todo!()
}

pub async fn edit_user_address(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(address_id): Path<ObjectId>,
) -> HandlerResult {
    todo!()
}

pub async fn delete_user_address(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(address_id): Path<ObjectId>,
) -> HandlerResult {
    todo!()
}
