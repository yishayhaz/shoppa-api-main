use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{AggregateOptions, FindOneAndUpdateOptions, FindOneOptions};
use serde::Deserialize;
use shoppa_core::{
    constans,
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{Categories, EmbeddedDocument, InnerCategories, InnerInnerCategories},
        DBConection, Pagination, Sorter,
    },
};

#[async_trait]
pub trait CategoriesFunctions {
    async fn get_nested_categories(
        &self,
        categorie_id: &ObjectId,
        innercategorie_id: &ObjectId,
        innerinnercategorie_id: &ObjectId,
        options: Option<FindOneOptions>,
    ) -> Result<Option<(Categories, InnerCategories, InnerInnerCategories)>>;
}

#[async_trait]
impl CategoriesFunctions for DBConection {
    async fn get_nested_categories(
        &self,
        categorie_id: &ObjectId,
        innercategorie_id: &ObjectId,
        innerinnercategorie_id: &ObjectId,
        // If in the future we need more option update this function
        _options: Option<FindOneOptions>,
    ) -> Result<Option<(Categories, InnerCategories, InnerInnerCategories)>> {
        let options = FindOneOptions::builder()
            .projection(doc! {
                "categories.$": 1,
                Categories::fields().created_at: 1,
                Categories::fields().updated_at: 1,
                Categories::fields().name: 1,
                Categories::fields().allowed_variants: 1
            })
            .build();

        let filters = doc! {
            "_id": categorie_id,
            "categories": {
                "$elemMatch": {
                    "_id": innercategorie_id,
                    "categories": {
                        "$elemMatch": {
                            "_id": innerinnercategorie_id
                        }
                    }
                }
            }
        };

        let category = self.get_category(filters, Some(options), None).await?;

        if let Some(mut category) = category {
            // we can safly unwrap since the above query will make sure that we get one category,
            // and only the one we need
            let inner_category = category.categories.pop().unwrap();

            // In the category.categories.categories we can have multiple categories
            // but the one we need will be there for sure, so we can filter and safely unwrap
            
            let inner_inner_category = inner_category
                .categories
                .into_iter()
                .filter(|inner_inner_category| inner_inner_category.id() == innerinnercategorie_id)
                .collect::<Vec<InnerInnerCategories>>()
                .pop()
                .unwrap();

            return Ok(Some((category, inner_category, inner_inner_category)));
        }

        Ok(None)
    }
}
