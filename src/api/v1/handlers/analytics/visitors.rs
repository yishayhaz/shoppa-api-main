use crate::helpers::{cookies::CookieManager, types::Cookeys};
use crate::{db::AxumDBExtansion, prelude::*};
use axum::response::IntoResponse;
use shoppa_core::{extractors::ClientIpAddress, ResponseBuilder};
use tower_cookies::Cookies;

pub async fn add_new_visitor_to_counter(
    db: AxumDBExtansion,
    cookies: Cookies,
    ClientIpAddress(ip): ClientIpAddress,
) -> HandlerResult {
    let cookie_key = Cookeys::VisitIndicator.to_string();

    match cookies.get(cookie_key.as_str()) {
        Some(_) => {}
        None => {
            db.insert_new_site_visit(ip, None, None).await?;
            cookies.set_cookie(
                &Cookeys::VisitIndicator,
                String::from("visited=true"),
                // one day
                86400,
                true,
            );
        }
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn get_views_count(db: AxumDBExtansion) -> HandlerResult {
    let views_count = db.count_site_visits(None, None, None).await?;

    Ok(ResponseBuilder::success(Some(views_count), None, None).into_response())
}
