use super::prelude::*;
use models::User;
type GetUserResult = Result<Option<User>, Response>;

async fn get_user(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetUserResult {
    let user =
        db.users.find_one(filter, option).await.map_err(|e| {
            ResponseBuilder::query_error(User::get_collection_name(), e).into_response()
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
