use super::super::prelude::routes::*;
use super::types::CreateProductPayload;
use crate::db::{inserts, queries, inserts::InsertDocumentErrors};

pub async fn create_new_product(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResponse {
    let categories = queries::get_category_hierarchy_for_subsubcategory(
        &db,
        // we can safely unwrap since the CreateProductPayload validate the length of the catagories
        payload.catagories.get(0).unwrap(),
        payload.catagories.get(1).unwrap(),
        payload.catagories.get(2).unwrap(),
    )
    .await?;

    if categories.is_none() {
        return  Err(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let categories = categories.unwrap();

    let store = queries::get_store_by_id(
        &db,
        &payload.store
    )
    .await?;

    if store.is_none() {
        return  Err(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let store = store.unwrap();

    let product = inserts::new_product(
        &db,
        &store,
        payload.brand,
        payload.description,
        payload.keywords.unwrap_or(vec![]),
        payload.name,
        &categories,
        categories.categories.get(0).unwrap(),
        categories.categories.get(0).unwrap().categories.get(0).unwrap(),
        payload.variants.unwrap_or(vec![]),
    ).await;

    match product {
        Ok(v) => Ok(ResponseBuilder::success(Some(v), None, None).into_response()),
        Err(e) => match e {
            InsertDocumentErrors::UnknownError => {
                return Err(ResponseBuilder::<u16>::error("", None, None, None).into_response());
            }
            _ => return Err(e.into_response()),
        },
    }

}