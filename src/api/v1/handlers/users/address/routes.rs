use super::types::{AddUserAddress, EditUserAddress};
use crate::{
    api::v1::middlewares::CurrentUser,
    db::{AxumDBExtansion, UserFunctions},
    helpers::cookies::CookieManager,
    prelude::*,
};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use shoppa_core::{extractors::JsonWithValidation, ResponseBuilder};
use tower_cookies::Cookies;

pub async fn get_user_addresses(
    db: AxumDBExtansion,
    cookies: Cookies,
    mut current_user: CurrentUser,
) -> HandlerResult {
    current_user.fetch(&db, None).await?;

    if !current_user.user_exists() {
        cookies.delete_access_cookie();
        return Ok(
            ResponseBuilder::<()>::error("UserNotFound", None, None, Some(404)).into_response(),
        );
    };

    Ok(
        ResponseBuilder::success(Some(current_user.user().unwrap().addresses), None, None)
            .into_response(),
    )
}

pub async fn add_user_address(
    db: AxumDBExtansion,
    cookies: Cookies,
    current_user: CurrentUser,
    JsonWithValidation(payload): JsonWithValidation<AddUserAddress>,
) -> HandlerResult {
    let update_res = db
        .add_user_address(&current_user.user_id, payload, None)
        .await?;

    if update_res.modified_count == 0 {
        if update_res.matched_count == 0 {
            cookies.delete_access_cookie();
            return Ok(
                ResponseBuilder::<()>::error("UserNotFound", None, None, Some(404)).into_response(),
            );
        }
        return Ok(
            ResponseBuilder::<()>::error("FaildToAddAddress", None, None, Some(500))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn edit_user_address(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(address_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<EditUserAddress>,
) -> HandlerResult {
    let update_res = db
        .edit_user_address(&current_user.user_id, &address_id, payload, None)
        .await?;

    if update_res.modified_count == 0 {
        if update_res.matched_count == 0 {
            return Ok(ResponseBuilder::<()>::error(
                "UserORAddressNotFound",
                None,
                None,
                Some(404),
            )
            .into_response());
        }
        return Ok(
            ResponseBuilder::<()>::error("FaildToEditAddress", None, None, Some(500))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn delete_user_address(
    db: AxumDBExtansion,
    cookies: Cookies,
    current_user: CurrentUser,
    Path(address_id): Path<ObjectId>,
) -> HandlerResult {
    let update_res = db
        .delete_user_address(&current_user.user_id, &address_id, None)
        .await?;

    if update_res.modified_count == 0 {
        if update_res.matched_count == 0 {
            cookies.delete_access_cookie();
            return Ok(
                ResponseBuilder::<()>::error("UserNotFound", None, None, Some(404)).into_response(),
            );
        }
        return Ok(
            ResponseBuilder::<()>::error("FaildToDeleteAddress", None, None, Some(500))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}
