use crate::{
    db::{AxumDBExtansion, UserFunctions},
    helpers::cookies::CookieManager,
    prelude::*,
    tokens::USER_TOKEN_MANAGER,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{
        models::{DBModel, User},
        populate::UsersPopulate,
        DBConection,
    },
    ResponseBuilder,
};
use std::sync::Arc;
use tower_cookies::Cookies;

// Use this struct to get the current user data in the request handler
// This will work only in the context of the login_required middleware
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: ObjectId,
    pub token_secret: String,
    pub guest: bool,
    user: Option<User>,
    user_exists: bool,
}

pub async fn login_required<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> StdResult<Response, Response> {
    let cookies = req.extensions().get::<Cookies>().ok_or(
        ResponseBuilder::error("", Some(()), Some("FAILD TO GET COOKIES"), Some(500))
            .into_response(),
    )?;

    let access_cookie = &cookies
        .get_access_cookie()
        .map_err(|e| e.into_response())?
        .ok_or(ResponseBuilder::error("", Some(()), None, Some(401)).into_response())?;

    if let Ok(data) = USER_TOKEN_MANAGER.decode_token(access_cookie) {
        req.extensions_mut()
            .insert(CurrentUser::new(data.user_id, data.secret, data.guest));

        Ok(next.run(req).await)
    } else {
        cookies.delete_access_cookie();
        Err(ResponseBuilder::error("", Some(()), None, Some(403)).into_response())
    }
}

pub async fn login_required_or_create_guest<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> StdResult<Response, Error> {
    let cookies = req
        .extensions()
        .get::<Cookies>()
        .ok_or(Error::Static("FAILD TO GET COOKIES"))?;

    if let Some(access_cookie) = &cookies.get_access_cookie()? {
        if let Ok(data) = USER_TOKEN_MANAGER.decode_token(access_cookie) {
            req.extensions_mut()
                .insert(CurrentUser::new(data.user_id, data.secret, data.guest));

            return Ok(next.run(req).await);
        }
    }

    let db = req
        .extensions()
        .get::<Arc<DBConection>>()
        .ok_or(Error::Static(
            "FAILD TO GET DB CONNECTION FROM REQUEST EXTENSIONS",
        ))?;

    let user = db.insert_new_user(User::new_guest(), None, None).await?;

    cookies.set_access_cookie(&user)?;

    // let token_data = USER_TOKEN_MANAGER.decode_token(cookies.get_access_cookie()?.unwrap().as_ref())?;

    let mut current_user = CurrentUser::new(user.id().unwrap().clone(), "".to_string(), true);

    current_user.set_user(user);

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> StdResult<Self, Self::Rejection> {
        parts
            .extensions
            .remove::<CurrentUser>()
            .ok_or(ResponseBuilder::error("", Some(()), None, Some(500)).into_response())
    }
}

impl CurrentUser {
    fn new(user_id: ObjectId, token_secret: String, guest: bool) -> Self {
        Self {
            user_id,
            token_secret,
            guest,
            user: None,
            user_exists: false,
        }
    }

    pub async fn fetch(
        &mut self,
        db: &AxumDBExtansion,
        populate: Option<UsersPopulate>,
    ) -> Result<()> {
        if self.user.is_none() {
            self.user = db
                .get_user_by_id_and_not_deleted_or_banned(&self.user_id, None, populate)
                .await?;
        }

        if self.user.is_none() {
            self.user_exists = false;
        } else {
            self.user_exists = true;
        };

        Ok(())
    }

    pub fn user_exists(&self) -> bool {
        self.user_exists
    }

    pub fn get_user_unchecked(&self) -> &User {
        self.user.as_ref().unwrap()
    }

    pub fn get_user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }
}
