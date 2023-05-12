use super::prelude::*;
use crate::{db::models::User, prelude::*};

type InsertUserResult = Result<User>;

pub async fn new_level_2_user(
    db: &DBExtension,
    email: String,
    password: String,
    name: String,
) -> InsertUserResult {
    let mut user = User::new(Some(email), Some(password), Some(name), 2);

    let res = db
        .users
        .insert_one(&user, None)
        .await
        .map_err(|e| Error::DBError(("users", e)))?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    user.update_id(id);

    Ok(user)
}
