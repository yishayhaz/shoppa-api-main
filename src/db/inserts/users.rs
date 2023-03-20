use super::{extract_insert_document_error, InsertDocumentErrors};
use crate::{
    db::models::{DBModel, User},
    helpers::types::DBExtension,
};

type InsertUserResult = Result<User, InsertDocumentErrors>;

pub async fn new_level_2_user(
    db: &DBExtension,
    email: String,
    password: String,
    name: String
) -> InsertUserResult {
    let mut user = User::new(Some(email), Some(password), Some(name), 2);

    let res = match db
        .users
        .insert_one(&user, None)
        .await
    {
        Ok(v) => v,
        Err(err) => return Err(extract_insert_document_error(*err.kind)),
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    user.update_id(id);

    Ok(user)
}
