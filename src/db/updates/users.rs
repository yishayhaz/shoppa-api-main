use crate::{db::models, helpers::types::DBExtension, prelude::*};
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateUserResult = Result<Option<models::User>>;

async fn update_user(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateUserResult {
    let user = db
        .users
        .find_one_and_update(filter, update, option)
        .await
        .map_err(|e| Error::DBError(("users", e)))?;

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
        "$set": {
            "email": email,
            "password": password,
            "name": name,
            "level": 2
        }
    };

    update_user(db, filter, update, None).await
}

pub async fn update_user_password(
    db: &DBExtension,
    user_id: &ObjectId,
    new_password: &str,
) -> UpdateUserResult {
    let filter = doc! {
        "_id": user_id
    };

    let update = doc! {
        "$set": {
            "password": new_password,
        }
    };

    update_user(db, filter, update, None).await
}
