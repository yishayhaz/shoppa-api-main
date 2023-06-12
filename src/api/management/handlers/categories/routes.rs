use super::types::CreateCatgoryPayload;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::{extract::Json, response::IntoResponse};
use bson::doc;
use shoppa_core::{
    db::models::{Category, Variants},
    ResponseBuilder,
};

pub async fn create_new_catagory(
    db: AxumDBExtansion,
    Json(payload): Json<CreateCatgoryPayload>,
) -> HandlerResult {
    if let Some(variants) = &payload.variants {
        let count = db
            .count_variants(
                Some(doc! {
                    Variants::fields().id: {
                        "$in": variants
                    }
                }),
                None,
            )
            .await?;

        if count != variants.len() as u64 {
            return Ok(ResponseBuilder::<u16>::error(
                "",
                None,
                Some("One or more variants doesnt exist"),
                Some(404),
            )
            .into_response());
        }
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
