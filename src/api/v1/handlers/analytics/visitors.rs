use crate::db::inserts;
use crate::helpers::cookies::create_cookie;
use crate::helpers::types::{Cookeys, DBExtension, HandlerResponse, ResponseBuilder};
use axum::response::IntoResponse;
use tower_cookies::Cookies;

pub async fn add_new_visitor_to_counter(db: DBExtension, cookies: Cookies) -> HandlerResponse {
    let cookie_key = Cookeys::VisitIndicator.get();

    match cookies.get(cookie_key) {
        Some(_) => {}
        None => {
            let cookie = create_cookie(
                &Cookeys::VisitIndicator,
                "visited=true",
                // one day
                86400.0,
                true,
            );

            cookies.add(cookie);
        }
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
