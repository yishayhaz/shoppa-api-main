use super::types::CreateProductPayload;
use crate::{
    api::v1::middlewares::OnlyInDev,
    db::{inserts, inserts::InsertDocumentErrors, queries},
    prelude::{handlers::*, *},
};

pub async fn create_new_product(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResult {
    let categories = queries::get_category_hierarchy_for_subsubcategory(
        &db,
        // we can safely unwrap since the CreateProductPayload validate the length of the categories
        payload.categories.get(0).unwrap(),
        payload.categories.get(1).unwrap(),
        payload.categories.get(2).unwrap(),
    )
    .await?;

    if categories.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let categories = categories.unwrap();

    let store = queries::get_store_by_id(&db, &payload.store).await?;

    if store.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let store = store.unwrap();

    let inner_category = categories.categories.get(0).unwrap();

    let product = inserts::new_product(
        &db,
        &store,
        payload.brand,
        payload.description,
        payload.keywords.unwrap_or(vec![]),
        payload.name,
        &categories,
        inner_category,
        inner_category.categories.get(0).unwrap(),
        payload.variants.unwrap_or(vec![]),
    )
    .await;

    match product {
        Ok(v) => Ok(ResponseBuilder::success(Some(v), None, None).into_response()),
        Err(e) => match e {
            InsertDocumentErrors::UnknownError => {
                return Ok(ResponseBuilder::<u16>::error("", None, None, None).into_response());
            }
            _ => return Ok(e.into_response()),
        },
    }
}
