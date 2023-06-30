use super::prelude::*;
use crate::prelude::*;
use models::User;

type GetUserResult = Result<Option<User>>;

async fn get_user(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetUserResult {
    let user =
        db.users.find_one(filter, option).await.map_err(|e| {
            Error::DBError(("users", e))
        })?;

    Ok(user)
}

pub async fn get_user_by_email(db: &DBExtension, email: &String) -> GetUserResult {
    let filter = doc! {
        "email": email,
    };

    get_user(db, filter, None).await
}

pub async fn get_user_by_id(db: &DBExtension, id: &ObjectId) -> GetUserResult {
    let filter = doc! {
        "_id": id,
    };

    get_user(db, filter, None).await
}