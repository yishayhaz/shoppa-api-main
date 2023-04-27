use crate::{
    api::v1::middlewares::*,
    db::inserts,
    helpers::{cookies::create_cookie, types::Cookeys},
    prelude::{handlers::*, *},
};

pub async fn add_new_visitor_to_counter(
    db: DBExtension,
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

            let _ = inserts::new_site_visit(&db, ip.to_string()).await;

            cookies.add(cookie);
        }
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
