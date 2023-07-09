use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneAndUpdateOptions, FindOneOptions};
use shoppa_core::db::{
    models::{User, UserStatus},
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

    // async fn create_new_guest_user(
    //     &self,
    //     email: &str,
    //     password: &str,
    //     options: Option<FindOneAndUpdateOptions>,
    // ) -> Result<Option<User>>;
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

    //     async fn update_user_to_level_2(
    //         &self,
    //         user_id: &ObjectId,
    //         email: &String,
    //         password: &String,
    //         name: &String,
    //         options: Option<FindOneAndUpdateOptions>,
    //     ) -> Result<Option<User>> {
    //         let filters = doc! {
    //             User::fields().id: user_id,
    //             User::fields().level: 1
    //         };

    //         let update = doc! {
    //             "$set": {
    //                 User::fields().email: email,
    //                 User::fields().password: password,
    //                 User::fields().name: name,
    //                 User::fields().level: 2
    //             }
    //         };

    //         self.find_and_update_user(filters, update, options, None).await
    //     }
}
