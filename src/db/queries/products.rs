use super::prelude::*;

type GetProductsExternalResult = Result<Vec<Document>, Response>;

// async fn get_product(
//     db: &DBExtension,
//     filter: Document,
//     option: Option<FindOneOptions>,
// ) -> GetStoreResult {
//     let store = match db.stores.find_one(filter, option).await {
//         Ok(store) => store,
//         Err(_) => {
//             return Err(ResponseBuilder::<u16>::error(
//                 // TODO add error code here
//                 "",
//                 None,
//                 Some("Internal Server Error while fetching store"),
//                 Some(500),
//             )
//             .into_response())
//         }
//     };

//     Ok(store)
// }

// pub async fn get_product_by_id(db: &DBExtension, id: &ObjectId) -> GetStoreResult {
//     let filter = doc! {
//         "_id": id,
//     };

//     get_product(db, filter, None).await
// }

pub async fn get_products_for_extarnel(db: &DBExtension) -> GetProductsExternalResult {
    let pipeline = [
        aggregations::match_query(doc! {}),
        aggregations::project(
            ProjectIdOptions::ToString,
            vec!["brand", "name", "keywords", "store.name"],
            Some(doc! {
                "categories": {
                "$map": {
                    "input": "$categories",
                    "in": {"_id":{"$toString":  "$$this._id"}, "name": "$$this.name"}
                    }
                },
                "store._id": aggregations::convert_to_string_safe("$store._id"),
                "created_at": aggregations::convert_to_string_safe("$created_at")
            }),
        ),
    ];

    let cursor = match db.products.aggregate(pipeline, None).await {
        Ok(v) => v,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching products"),
                Some(500),
            )
            .into_response());
        }
    };

    match consume_cursor(cursor).await {
        Ok(products) => Ok(products),
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching products"),
                Some(500),
            )
            .into_response());
        }
    }
}
