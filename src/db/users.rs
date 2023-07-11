use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::{
    options::{FindOneAndUpdateOptions, FindOneOptions, UpdateOptions},
    results::UpdateResult,
};
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
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<CartItem> + Send + Sync;

    async fn remove_product_from_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;

    async fn edit_product_in_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        quantity: u32,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;
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
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<CartItem> + Send + Sync,
    {
        let cart_item: CartItem = cart_item.into();

        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
            // Check if the product is already in the cart
            // If it is, then we don't need to add it again
            User::fields().cart(true).items: {
                "$not": {
                    "$elemMatch": {
                        User::fields().cart(false).items(false).product: cart_item.product_id(),
                        User::fields().cart(false).items(false).item_id: &cart_item.item_id,
                    }
                }
            }
        };

        let update = doc! { "$push": { User::fields().cart(true).items: cart_item } };

        self.update_user(filters, update, options, None).await
    }

    async fn remove_product_from_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        // by filtering only by this two fields,
        // if the matched count is 0, that means the user
        // got deleted or banned, so we can remove his access token
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
        };

        let update = doc! {
            "$pull": {
                User::fields().cart(true).items: {
                    User::fields().cart(false).items(false).product: product_id,
                    User::fields().cart(false).items(false).item_id: item_id,
                }
            }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn edit_product_in_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        quantity: u32,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
            User::fields().cart(true).items: {
                "$elemMatch": {
                    User::fields().cart(false).items(false).product: product_id,
                    User::fields().cart(false).items(false).item_id: item_id,
                }
            }
        };

        let update = doc! {
            "$set": {
                format!("{}.$.{}",
                    User::fields().cart(true).items,
                    User::fields().cart(false).items(false).quantity
                ): quantity
            }
        };

        self.update_user(filters, update, options, None).await
    }
}
