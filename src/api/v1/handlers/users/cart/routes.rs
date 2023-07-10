use super::types::{AddProductToCartPayload, EditProductInCartPayload, RemoveProductFromCartQuery};
use crate::api::v1::middlewares::CurrentUser;
use crate::helpers::cookies::CookieManager;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::{
    extract::{Json, Query},
    response::IntoResponse,
};
use shoppa_core::ResponseBuilder;
use shoppa_core::{
    db::models::{EmbeddedDocument, ProductItemStatus, ProductStatus},
    extractors::JsonWithValidation,
};
use tower_cookies::Cookies;

pub async fn add_product_to_cart(
    db: AxumDBExtansion,
    cookies: Cookies,
    mut current_user: CurrentUser,
    JsonWithValidation(payload): JsonWithValidation<AddProductToCartPayload>,
) -> HandlerResult {
    current_user.fetch(&db, None).await?;

    if !current_user.user_exists() {
        cookies.delete_access_cookie();
        return Ok(
            ResponseBuilder::<()>::error("User not found", None, None, None).into_response(),
        );
    }

    let product = db
        .get_product_by_id(&payload.product_id, None, None, None)
        .await?;

    if product.is_none() {
        return Ok(
            ResponseBuilder::<()>::error("Product not found", None, None, None).into_response(),
        );
    }

    let product = product.unwrap();

    if product.status != ProductStatus::Active {
        match product.status {
            ProductStatus::Inactive => {
                return Ok(ResponseBuilder::<()>::error(
                    "Product is currently not availble",
                    None,
                    None,
                    None,
                )
                .into_response());
            }
            _ => {
                return Ok(
                    ResponseBuilder::<()>::error("Product not found", None, None, None)
                        .into_response(),
                )
            }
        };
    };

    // check if item exists in product and also check if it is available
    let item = product
        .items
        .iter()
        .find(|item| *item.id() == payload.item_id && item.status != ProductItemStatus::Deleted);

    if item.is_none() {
        return Ok(
            ResponseBuilder::<()>::error("Product item not found", None, None, None)
                .into_response(),
        );
    }

    let item = item.unwrap();

    if item.status != ProductItemStatus::Active {
        return Ok(ResponseBuilder::<()>::error(
            "Product item is currently not availble",
            None,
            None,
            None,
        )
        .into_response());
    };

    if item.in_storage < payload.quantity as u64 {
        return Ok(ResponseBuilder::error(
            "Not enough items in storage",
            Some(item.in_storage),
            None,
            None,
        )
        .into_response());
    }

    let update_quantity = current_user
        .get_user_unchecked()
        .cart
        .items
        .iter()
        .find(|item| item.product_id() == &payload.product_id && item.item_id == payload.item_id)
        .is_some();

    if update_quantity {
        // TODO update quantity in cart (same as edit_product_in_cart)
    }else {
        // TODO add product to cart
    }

    Ok(().into_response())
}

pub async fn get_full_cart(db: AxumDBExtansion, current_user: CurrentUser) -> HandlerResult {
    todo!()
    // let cart = db.get_cart_by_user_id(&current_user.user_id).await?;

    // Ok(ResponseBuilder::success(cart, None, None).into_response())
}

pub async fn remove_product_from_cart(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Query(query): Query<RemoveProductFromCartQuery>,
) -> HandlerResult {
    todo!()
}

pub async fn edit_product_in_cart(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Json(payload): Json<EditProductInCartPayload>,
) -> HandlerResult {
    todo!()
}
