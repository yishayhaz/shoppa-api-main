use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use shoppa_core::db::{aggregations, models::Category, DBConection};

#[async_trait]
pub trait CategoriesFunctions {
    async fn get_categories_for_external(&self, parent: Option<ObjectId>) -> Result<Vec<Document>>;
}

#[async_trait]
impl CategoriesFunctions for DBConection {
    async fn get_categories_for_external(&self, parent: Option<ObjectId>) -> Result<Vec<Document>> {
        let filters = doc! {
            // if parent is None then get all the root categories,
            // we have document validation so the root categories will parent as null
            Category::fields().parent: parent
        };

        let pipeline = [
            aggregations::match_query(&filters),
            aggregations::project(
                aggregations::ProjectIdOptions::Keep,
                [
                    Category::fields().name,
                    Category::fields().parent,
                    Category::fields().children,
                    Category::fields().ancestors,
                ],
                None,
            ),
        ];

        self.aggregate_categories(pipeline, None).await
    }
}
