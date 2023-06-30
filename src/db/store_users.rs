use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneAndUpdateOptions;
use shoppa_core::db::aggregations;
use shoppa_core::db::models::Store;
use shoppa_core::db::{models::StoreUser, DBConection};

#[async_trait]
pub trait StoreUserFunctions {
    async fn complete_store_user_registration(
        &self,
        user_id: &ObjectId,
        registration_token_secret: String,
        password: String,
        name: Option<String>,
    ) -> Result<Option<StoreUser>>;

    async fn get_store_user_by_email(
        &self,
        email: &str,
        registration_completed: bool,
    ) -> Result<Option<StoreUser>>;
}

#[async_trait]
pub trait StoreUserFunctionsForStoreUser {
    async fn get_me(&self, user_id: &ObjectId) -> Result<Option<Document>>;
}

#[async_trait]
impl StoreUserFunctions for DBConection {
    async fn complete_store_user_registration(
        &self,
        user_id: &ObjectId,
        registration_token_secret: String,
        password: String,
        name: Option<String>,
    ) -> Result<Option<StoreUser>> {
        let filters = doc! {
            StoreUser::fields().id: user_id,
            StoreUser::fields().registration_completed: false,
            StoreUser::fields().registration_token_secret: registration_token_secret,
        };

        let update = doc! {
            "$set": {
                StoreUser::fields().password: password,
                StoreUser::fields().registration_completed: true,
                StoreUser::fields().name: name,
                StoreUser::fields().registration_token_secret: None::<String>,
            },
            "$currentDate": {
                StoreUser::fields().registration_completed_at: true,
            }
        };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        self.find_and_update_store_user(filters, update, Some(options), None)
            .await
    }

    async fn get_store_user_by_email(
        &self,
        email: &str,
        registration_completed: bool,
    ) -> Result<Option<StoreUser>> {
        let filters = doc! {
            StoreUser::fields().email: email,
            StoreUser::fields().registration_completed: registration_completed,
        };

        self.get_store_user(filters, None, None, None).await
    }
}

#[async_trait]
impl StoreUserFunctionsForStoreUser for DBConection {
    async fn get_me(&self, user_id: &ObjectId) -> Result<Option<Document>> {
        let pipeline = [
            aggregations::match_query(&doc! {
                StoreUser::fields().id: user_id,
            }),
            aggregations::lookup::<Store>(
                StoreUser::fields().store,
                Store::fields().id,
                StoreUser::fields().store,
                Some(vec![aggregations::project(
                    aggregations::ProjectIdOptions::Keep,
                    [Store::fields().logo, Store::fields().name],
                    None,
                )]),
                None,
            ),
            aggregations::project(aggregations::ProjectIdOptions::Keep, [
                StoreUser::fields().id,
                StoreUser::fields().store,
                StoreUser::fields().name,
                StoreUser::fields().email,
                StoreUser::fields().registration_completed_at,
            ], None),
        ];

        let mut store_user = self.aggregate_store_users(pipeline, None, None).await?;

        Ok(store_user.pop())
    }
}
