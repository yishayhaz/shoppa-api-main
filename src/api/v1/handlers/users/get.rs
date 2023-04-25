use crate::{
    db::queries,
    helpers::{cookies::delete_cookie, types::Cookeys},
    prelude::{handlers::*, *},
    api::v1::middlewares::*,
};

pub async fn get_me(
    db: DBExtension,
    cookies: Cookies,
    GetTokenForGetMe(token_data): GetTokenForGetMe,
) -> HandlerResult {


    let user = queries::get_user_by_id(&db, &token_data.user_id).await?;

    match user {
        Some(user) => {
            Ok(ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response())
        }
        // TODO add error code
        None => {
            cookies.remove(delete_cookie(&Cookeys::AccessToken));

            Ok(ResponseBuilder::<u16>::error("", None, None, None).into_response())
        }
    }
}