use crate::{
    helpers::cookies::delete_cookie, helpers::types::Cookeys, tokens::STORE_USER_TOKEN_MANAGER,
};
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shoppa_core::ResponseBuilder;
use tower_cookies::Cookies;
use bson::oid::ObjectId;


pub struct CurrentUser {
    pub user_id: ObjectId,
    pub token_secret: String,
    pub store_id: ObjectId,
}

pub async fn login_required<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let cookies = req.extensions().get::<Cookies>().ok_or(
        ResponseBuilder::error("", Some(()), Some("FAILD TO GET COOKIES"), Some(500))
            .into_response(),
    )?;

    let access_cookie = &cookies
        .get(Cookeys::StoreUserAccessToken.to_string().as_str())
        .ok_or(ResponseBuilder::error("", Some(()), None, Some(401)).into_response())?;

    let token_data = STORE_USER_TOKEN_MANAGER.decode_token(access_cookie.value());

    if let Ok(data) = token_data {

        req.extensions_mut().insert(CurrentUser { 
            user_id: data.user_id,
            token_secret: data.token_secret,
            store_id: data.store_id,
        });

        Ok(next.run(req).await)
    } else {
        cookies.remove(delete_cookie(&Cookeys::StoreUserAccessToken));

        Err(ResponseBuilder::error("", Some(()), None, Some(403)).into_response())
    }
}
