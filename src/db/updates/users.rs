use crate::{
    db::models,
    helpers::types::{DBExtension, ResponseBuilder},
};
use axum::response::IntoResponse;
use axum::response::Response;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateUserResult = Result<Option<models::User>, Response>;

async fn _update_user(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateUserResult {
    let user = match db.users.find_one_and_update(filter, update, option).await {
        Ok(user) => user,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching user"),
                Some(500),
            )
            .into_response());
        }
    };

    Ok(user)
}

pub async fn update_user_to_level_2(
    db: &DBExtension,
    user_id: &ObjectId,
    email: &String,
    password: &String,
    name: &String,
) -> UpdateUserResult {
    let filter = doc! {
        "_id": user_id,
        "level": 1
    };

    let update = doc! {
        "email": email,
        "password": password,
        "name": name,
        "level": 2
    };

    _update_user(db, filter, update, None).await
}
