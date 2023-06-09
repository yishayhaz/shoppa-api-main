use crate::helpers::{cookies::create_cookie, types::Cookeys};
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
            let cookie = create_cookie(
                &Cookeys::VisitIndicator,
                String::from("visited=true"),
                // one day
                86400.0,
                true,
            );

            db.insert_new_site_visit(ip, None).await;

            cookies.add(cookie);
        }
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn get_views_count(db: AxumDBExtansion) -> HandlerResult {
    let views_count = db.count_site_visits(None, None).await?;

    Ok(ResponseBuilder::success(Some(views_count), None, None).into_response())
}
