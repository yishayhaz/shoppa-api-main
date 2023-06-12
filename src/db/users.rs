use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneAndUpdateOptions, FindOneOptions};
use shoppa_core::db::{models::User, populate::UsersPopulate, DBConection};

#[async_trait]
pub trait UserFunctions {
    async fn get_user_by_email(
        &self,
        email: &str,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>>;

    async fn update_user_password(
        &self,
        user_id: &ObjectId,
        new_password: &str,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>>;

    async fn update_user_to_level_2(
        &self,
        user_id: &ObjectId,
        email: &String,
        password: &String,
        name: &String,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>>;
}

#[async_trait]
pub trait UserAdminFunctions {}

#[async_trait]
impl UserFunctions for DBConection {
    async fn get_user_by_email(
        &self,
        email: &str,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>> {
        let filters = doc! { User::fields().email: email };

        self.get_user(filters, options, populate).await
    }

    async fn update_user_password(
        &self,
        user_id: &ObjectId,
        new_password: &str,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>> {
        let update = doc! { "$set": { User::fields().password: new_password } };

        self.find_and_update_user_by_id(user_id, update, options)
            .await
    }
    async fn update_user_to_level_2(
        &self,
        user_id: &ObjectId,
        email: &String,
        password: &String,
        name: &String,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>> {
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().level: 1
        };

        let update = doc! {
            "$set": {
                User::fields().email: email,
                User::fields().password: password,
                User::fields().name: name,
                User::fields().level: 2
            }
        };

        self.find_and_update_user(filters, update, options).await
    }
}
