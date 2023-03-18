use crate::{
    db::models,
    helpers::types::{DBExtension, ResponseBuilder},
};
use axum::response::IntoResponse;
use axum::response::Response;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneOptions;

type GetUserResult = Result<Option<models::User>, Response>;

async fn get_user(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetUserResult {
    let user = match db.users.find_one(filter, option).await {
        Ok(user) => user,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                None,
                Some(String::from("Internal Server Error while fetching user")),
                Some(500),
            )
            .into_response())
        }
    };

    Ok(user)
}

pub async fn get_user_by_email(db: &DBExtension, email: String) -> GetUserResult {
    let filter = doc! {
        "email": email,
    };

    get_user(db, filter, None).await
}

pub async fn get_user_by_id(db: &DBExtension, id: ObjectId) -> GetUserResult {
    let filter = doc! {
        "_id": id,
    };

    get_user(db, filter, None).await
}
