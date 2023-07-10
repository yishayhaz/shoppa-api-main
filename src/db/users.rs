use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneAndUpdateOptions, FindOneOptions};
use shoppa_core::db::{
    models::{CartItem, User, UserStatus},
    populate::UsersPopulate,
    DBConection,
};

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

    async fn get_user_by_id_and_not_deleted_or_banned(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>>;

    async fn add_product_to_cart<T>(
        &self,
        user_id: &ObjectId,
        cart_item: T,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>>
    where
        T: Into<CartItem> + Send + Sync;
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
        let filters = doc! { User::fields().email: email, User::fields().status: {
            "$nin": [UserStatus::Deleted, UserStatus::Banned]
        } };

        self.get_user(filters, options, populate, None).await
    }

    async fn update_user_password(
        &self,
        user_id: &ObjectId,
        new_password: &str,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>> {
        let update = doc! { "$set": { User::fields().password: new_password } };

        self.find_and_update_user_by_id(user_id, update, options, None)
            .await
    }

    async fn get_user_by_id_and_not_deleted_or_banned(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>> {
        let filters = doc! { User::fields().id: user_id, User::fields().status: {
            "$nin": [UserStatus::Deleted, UserStatus::Banned]
        } };

        self.get_user(filters, options, populate, None).await
    }

    async fn add_product_to_cart<T: Into<CartItem>>(
        &self,
        user_id: &ObjectId,
        cart_item: T,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>>
    where
        T: Into<CartItem> + Send + Sync,
    {
        let cart_item: CartItem = cart_item.into();

        let update = doc! { "$push": { User::fields().cart(true).items: cart_item } };

        todo!()
    }
}
