use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::options::FindOneOptions;
use shoppa_core::db::{models::CheckOutSession, populate::CheckoutSessionPopulate, DBConection};

#[async_trait]
pub trait CheckoutSessionFunctions {
    async fn get_checkout_session_by_user(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<CheckoutSessionPopulate>,
    ) -> Result<Option<CheckOutSession>>;
}

#[async_trait]
impl CheckoutSessionFunctions for DBConection {
    async fn get_checkout_session_by_user(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<CheckoutSessionPopulate>,
    ) -> Result<Option<CheckOutSession>> {
        let query = doc! {
            CheckOutSession::fields().user: user_id,
        };

        self.get_checkout_session(query, options, populate, None).await
    }
}
