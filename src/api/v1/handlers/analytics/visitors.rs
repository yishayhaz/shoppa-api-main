use crate::db::inserts;
use crate::helpers::{
    cookies::create_cookie,
    ip_address::ClientIpAddress,
    types::{Cookeys, DBExtension, HandlerResponse, ResponseBuilder},
};
use axum::response::IntoResponse;
use tower_cookies::Cookies;

pub async fn add_new_visitor_to_counter(
    db: DBExtension,
    cookies: Cookies,
    ClientIpAddress(ip): ClientIpAddress,
) -> HandlerResponse {
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

            let _ = inserts::new_site_visit(&db, ip.to_string()).await;

            cookies.add(cookie);
        }
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
