use super::CurrentUser;
use crate::{helpers::cookies::CookieManager, prelude::*, tokens::USER_TOKEN_MANAGER};
use axum::{http::Request, middleware::Next, response::Response};

use tower_cookies::Cookies;

pub async fn guest_required<B>(mut req: Request<B>, next: Next<B>) -> StdResult<Response, Error> {
    let cookies = req
        .extensions()
        .get::<Cookies>()
        .ok_or(Error::Static("FAILD TO GET COOKIES"))?;

    let mut current_user: Option<CurrentUser> = None;

    if let Some(access_cookie) = cookies.get_access_cookie() {
        if let Ok(data) = USER_TOKEN_MANAGER.decode_token(&access_cookie) {
            if !data.guest {
                return Err(Error::ApiErrorWithCode("Guest required", 401));
            }

            current_user = Some(CurrentUser::new(data.user_id, data.secret, data.guest));
        } else {
            cookies.delete_access_cookie();
        }
    }

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
