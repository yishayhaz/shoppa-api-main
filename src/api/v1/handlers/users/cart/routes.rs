use super::types::{AddProductToCartPayload, EditProductInCartPayload, RemoveProductFromCartQuery};
use crate::{
    api::v1::middlewares::CurrentUser,
    db::{AxumDBExtansion, UserFunctions},
    helpers::cookies::CookieManager,
    prelude::*,
};
use axum::{
    extract::{Json, Query},
    response::IntoResponse,
};
use bson::{doc, oid::ObjectId};
use serde_json::json;
use shoppa_core::{
    db::{
        models::{
            CheckOutSession, CheckOutSessionPart, CheckOutSessionPartItem, DBModel,
            EmbeddedDocument, ProductItemStatus, ProductStatus, Store,
        },
        populate::{FieldPopulate, UsersPopulate},
    },
    extractors::JsonWithValidation,
    ResponseBuilder,
};
use std::collections::HashMap;
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

    let updated_res;

    if update_quantity {
        updated_res = db
            .edit_product_in_cart(
                &current_user.user_id,
                &payload.product_id,
                &payload.item_id,
                payload.quantity,
                None,
            )
            .await?;
    } else {
        updated_res = db
            .add_product_to_cart(&current_user.user_id, payload, None)
            .await?
    }

    if updated_res.modified_count == 0 {
        if updated_res.matched_count == 0 {
            return Ok(ResponseBuilder::<()>::error(
                "Maybe item is in cart already?",
                None,
                None,
                None,
            )
            .into_response());
        }

        return Ok(
            ResponseBuilder::<()>::error("Failed to add product to cart", None, None, None)
                .into_response(),
        );
    }

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn get_full_cart(db: AxumDBExtansion, current_user: CurrentUser) -> HandlerResult {
    let cart = db.get_user_full_cart(&current_user.user_id, None).await?;

    Ok(ResponseBuilder::success(Some(cart), None, None).into_response())
}

pub async fn remove_product_from_cart(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    cookies: Cookies,
    Query(query): Query<RemoveProductFromCartQuery>,
) -> HandlerResult {
    let update_res = db
        .remove_product_from_cart(
            &current_user.user_id,
            &query.product_id,
            &query.item_id,
            None,
        )
        .await?;

    if update_res.modified_count == 0 {
        if update_res.matched_count == 0 {
            cookies.delete_access_cookie();
            return Ok(
                ResponseBuilder::<()>::error("User not found", None, None, None).into_response(),
            );
        }

        return Ok(
            ResponseBuilder::<()>::error("Maybe item not in cart", None, None, None)
                .into_response(),
        );
    }

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn edit_product_in_cart(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Json(payload): Json<EditProductInCartPayload>,
) -> HandlerResult {
    let updated_res = db
        .edit_product_in_cart(
            &current_user.user_id,
            &payload.product_id,
            &payload.item_id,
            payload.new_quantity,
            None,
        )
        .await?;

    if updated_res.modified_count == 0 {
        return Ok(ResponseBuilder::<()>::error(
            "User not found or item not in cart",
            None,
            None,
            None,
        )
        .into_response());
    }

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn start_checkout(
    db: AxumDBExtansion,
    mut current_user: CurrentUser,
    cookies: Cookies,
) -> HandlerResult {
    let populate = UsersPopulate {
        cart_products: FieldPopulate::Field,
        options: None,
    };

    current_user.force_fetch(&db, Some(populate)).await?;

    if !current_user.user_exists() {
        cookies.delete_access_cookie();
        return Ok(
            ResponseBuilder::<()>::error("User not found", None, None, None).into_response(),
        );
    }

    let mut checkout_session = CheckOutSession::new(current_user.user_id.clone());

    let user = current_user.user().unwrap();

    if user.cart.items.is_empty() {
        return Ok(ResponseBuilder::<()>::error("Cart is empty", None, None, None).into_response());
    }
    // If one is populated, all are populated
    if user.cart.items.get(0).unwrap().product.is_not_populated() {
        // This is not supposed to happen
        return Ok(
            ResponseBuilder::<()>::error("CartItemNotPopulated", None, None, Some(500))
                .into_response(),
        );
    }

    let mut checkout_parts: HashMap<&ObjectId, CheckOutSessionPart> = HashMap::new();

    let mut errors = Vec::new();

    // grouping items by store
    user.cart.items.iter().for_each(|item| {
        let product = item
            .product
            .as_populated()
            .expect("Product is not populated");

        let product_item = product
            .items
            .get(0)
            .expect("Product has no items in start checkout");

        let checkout_part =
            checkout_parts
                .entry(product.store_id())
                .or_insert_with(|| CheckOutSessionPart {
                    store: product.store_id().clone(),
                    items_total: 0.0,
                    delivery_cost: 0.0,
                    items: Vec::new(),
                    // In the future the user will send the desired delivery strategy
                    delivery_strategy: "default".to_string(),
                });

        if product.status != ProductStatus::Active {
            errors.push(json!({
                "product": product.id().unwrap(),
                "error": "Product is not active"
            }));
            return;
        }

        if product_item.status != ProductItemStatus::Active {
            errors.push(json!({
                "product": product.id().unwrap(),
                "item": product_item.id(),
                "error": "Product item is not active"
            }));
            return;
        }

        if item.quantity as u64 > product_item.in_storage {
            errors.push(json!({
                "product": product.id().unwrap(),
                "item": product_item.id(),
                "in_storage": product_item.in_storage,
                "error": "Not enough items in storage"
            }));
            return;
        }

        checkout_part.items.push(CheckOutSessionPartItem {
            product: product.id().unwrap().clone(),
            item_id: product_item.id().clone(),
            quantity: item.quantity,
            price: product_item.price,
        });

        checkout_part.items_total += product_item.price * item.quantity as f64;
    });

    if !errors.is_empty() {
        return Ok(ResponseBuilder::error("", Some(errors), None, None).into_response());
    }

    // getting stores from db
    let stores = db
        .get_stores(
            doc! {
                Store::fields().id: {
                    "$in": checkout_parts.keys().collect::<Vec<_>>()
                }
            },
            None,
            None,
            None,
        )
        .await?;

    // checking if all stores were found
    if stores.len() != checkout_parts.len() {
        return Ok(
            ResponseBuilder::<()>::error("Some stores not found", None, None, None).into_response(),
        );
    }

    // making sure all stores have a default delivery strategy
    for store in &stores {
        if store.delivery_strategies.default.is_none() {
            return Ok(ResponseBuilder::<()>::error(
                "Some stores have no delivery strategies",
                None,
                None,
                None,
            )
            .into_response());
        }
    }

    // Includes all products in cart + delivery
    let mut total_price = 0.0;

    // Adding delivery cost to each part
    let checkout_parts: Vec<CheckOutSessionPart> = checkout_parts
        .into_iter()
        .map(|(store_id, mut part)| {
            // Not possible to fail
            let store = stores
                .iter()
                .find(|store| store.id().unwrap() == store_id)
                .expect("Store not found");

            // checking if items total is above store min order
            if part.items_total < store.min_order as f64 {
                errors.push(json!({
                    "store": store.id().unwrap(),
                    "min_order": store.min_order,
                    "items_total": part.items_total,
                    "error": "Items total is below store min order"
                }));
            };

            // checking if store as a free above policy
            if let Some(free_above) = store
                .delivery_strategies
                .default
                .as_ref()
                .unwrap()
                .free_above
            {
                // if it does, check if the items total is above it
                if part.items_total < free_above as f64 {
                    part.delivery_cost = store.delivery_strategies.default.as_ref().unwrap().price;
                } else {
                    part.delivery_cost = 0.0;
                }
            }
            // if it doesn't, set the delivery cost to the provided price
            else {
                part.delivery_cost = store.delivery_strategies.default.as_ref().unwrap().price;
            }

            // adding delivery cost and total part items to total price
            total_price += part.items_total + part.delivery_cost;

            part
        })
        .collect();

    if !errors.is_empty() {
        return Ok(ResponseBuilder::error("", Some(errors), None, None).into_response());
    }

    checkout_session.parts = checkout_parts;
    checkout_session.total = total_price;

    let checkout_session = db
        .insert_new_checkout_session(checkout_session, None)
        .await?;

    cookies.set_checkout_session_cookie(&checkout_session)?;

    Ok(ResponseBuilder::success(Some(checkout_session), None, None).into_response())
}
