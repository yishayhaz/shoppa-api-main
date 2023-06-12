use super::types::CreateCatgoryPayload;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::{extract::Json, response::IntoResponse};
use shoppa_core::{db::models::Category, ResponseBuilder};

pub async fn create_new_catagory(
    db: AxumDBExtansion,
    Json(payload): Json<CreateCatgoryPayload>,
) -> HandlerResult {
    if let Some(_) = &payload.variants {
        todo!("validate variants ids")
    }

    let parent = match &payload.parent {
        Some(parent_id) => {
            let p = db.get_category_by_id(parent_id, None, None).await?;

            if p.is_none() {
                return Ok(ResponseBuilder::<u16>::error(
                    "",
                    None,
                    Some("The provided parent category doesnt exist"),
                    Some(404),
                )
                .into_response());
            }

            Some(p.unwrap())
        }
        None => None,
    };

    let new_category = Category::new(payload.name, payload.variants, parent.as_ref());

    let new_category = db.insert_new_category(new_category, None).await?;

    Ok(ResponseBuilder::success(Some(new_category), None, Some(200)).into_response())
}
