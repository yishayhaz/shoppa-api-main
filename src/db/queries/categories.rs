use super::prelude::*;
use crate::db::models::Categories;

type GetCategorieResult = Result<Option<models::Categories>, Response>;

async fn get_category(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetCategorieResult {
    let categorie = match db.categories.find_one(filter, option).await {
        Ok(categorie) => categorie,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching categorie"),
                Some(500),
            )
            .into_response());
        }
    };

    Ok(categorie)
}

pub async fn get_category_hierarchy_for_subsubcategory(
    db: &DBExtension,
    parent_category_id: &ObjectId,
    immediate_category_id: &ObjectId,
    direct_category_id: &ObjectId,
) -> GetCategorieResult {
    let options = FindOneOptions::builder()
        .projection(doc! {
            "categories.$": 1,
            "created_at": 1,
            "updated_at": 1,
            "name": 1,
            "allowed_variants": 1
        })
        .build();

    let filters = doc! {
        "_id": parent_category_id,
        "categories": {
            "$elemMatch": {
                "_id": immediate_category_id,
                "categories": {
                    "$elemMatch": {
                        "_id": direct_category_id
                    }
                }
            }
        }
    };

    match get_category(db, filters, Some(options)).await? {
        Some(mut category) => {
            // we can safly unwrap since the above query will make sure that we get one category, and only the one we need
            category
                .categories
                .get_mut(0)
                .unwrap()
                .categories
                .retain(|c| c.id() == direct_category_id);

            if category.categories.get(0).unwrap().categories.len() != 1 {
                return Ok(None);
            }

            return Ok(Some(category));
        }
        None => Ok(None),
    }
}

pub async fn get_categories_for_extarnel(
    db: &DBExtension,
    id: Option<ObjectId>,
    child_id: Option<ObjectId>,
) -> Result<Vec<Document>, Response> {
    let mut pipeline = vec![];

    if let Some(id) = id {
        // getting the parent sub categories
        pipeline.push(aggregations::match_query(&doc! {
            Categories::fields().id: id
        }));
        pipeline.push(aggregations::unwind(Categories::fields().categories, false));

        pipeline.push(aggregations::replace_root(Categories::fields().categories));

        // getting child sub categories
        if let Some(child_id) = child_id {
            pipeline.push(aggregations::match_query(&doc! {
                Categories::fields().id: child_id
            }));
            pipeline.push(aggregations::unwind(
                Categories::fields().categories().categories,
                false,
            ));

            pipeline.push(aggregations::replace_root(Categories::fields().categories().categories));
        }
    };
    pipeline.push(aggregations::project(
        ProjectIdOptions::ToString,
        [Categories::fields().name].to_vec(),
        Some(doc! {
            Categories::fields().created_at: aggregations::convert_to_string_safe("$created_at"),
            Categories::fields().updated_at: aggregations::convert_to_string_safe("$updated_at")
        }),
    ));
    
    let cursor = db.categories.aggregate(pipeline, None).await.map_err(|_| {
        ResponseBuilder::<u16>::error(
            // TODO add error code here
            "",
            None,
            Some("Internal Server Error while fetching categories"),
            Some(500),
        )
        .into_response()
    })?;

    Ok(consume_cursor(cursor).await.map_err(|_| {
        ResponseBuilder::<u16>::error(
            // TODO add error code here
            "",
            None,
            Some("Internal Server Error while consuming categories cursor"),
            Some(500),
        )
        .into_response()
    })?)
}
