use crate::{db::AxumDBExtansion, prelude::*};
use axum::response::IntoResponse;
use shoppa_core::ResponseBuilder;

pub async fn get_full_cart(db: AxumDBExtansion) -> HandlerResult {
    todo!()
    // let cart = db.get_cart_by_user_id(&current_user.user_id).await?;

    // Ok(ResponseBuilder::success(cart, None, None).into_response())
}

pub async fn add_product_to_cart(db: AxumDBExtansion) -> HandlerResult {
    todo!()
}

pub async fn remove_product_from_cart(db: AxumDBExtansion) -> HandlerResult {
    todo!()
}

pub async fn edit_product_in_cart(db: AxumDBExtansion) -> HandlerResult {
    todo!()
}
