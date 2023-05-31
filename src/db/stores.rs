use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::AggregateOptions;
use shoppa_core::db::{
    aggregations::{self, ProjectIdOptions},
    models, DBConection, Pagination,
};
use shoppa_core::Result;

#[async_trait]
pub trait StoreFunctions {
    async fn get_random_stores_names(
        &self,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;
    async fn get_stores_names_for_autocomplete(
        &self,
        free_text: String,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;
    async fn get_many_stores_for_extarnel(
        &self,
        pagination: Option<Pagination>,
        free_text: Option<String>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
    async fn get_store_for_extarnel(
        &self,
        store_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>>;
}

#[async_trait]
pub trait AdminStoreFunctions {
    async fn get_stores_for_admins(
        &self,
        pagination: Option<Pagination>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
}

#[async_trait]
impl StoreFunctions for DBConection {
    async fn get_random_stores_names(
        &self,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let pipeline = [
            aggregations::sample(10),
            aggregations::project(ProjectIdOptions::Keep, [models::Store::fields().name], None),
        ];

        self.aggregate_stores(pipeline, options).await
    }

    async fn get_stores_names_for_autocomplete(
        &self,
        free_text: String,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let pipeline = [
            aggregations::autocomplete_store_search(&free_text),
            aggregations::add_score_meta(),
            aggregations::sort_by_score(),
            aggregations::limit(10),
            aggregations::project(ProjectIdOptions::Keep, [models::Store::fields().name], None),
        ];

        self.aggregate_stores(pipeline, options).await
    }

    async fn get_many_stores_for_extarnel(
        &self,
        pagination: Option<Pagination>,
        free_text: Option<String>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let pipeline = [
            aggregations::search_store(&free_text, &vec![], None),
            aggregations::add_score_meta(),
            aggregations::sort_by_score(),
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    models::Store::fields().name,
                    models::Store::fields().logo(true).path,
                    models::Store::fields().logo(true).file_name,
                    models::Store::fields().logo(true).mime_type,
                    models::Store::fields().logo(true).file_type,
                    models::Store::fields().banner(true).path,
                    models::Store::fields().banner(true).file_name,
                    models::Store::fields().banner(true).mime_type,
                    models::Store::fields().banner(true).file_type,
                    models::Store::fields().description,
                    models::Store::fields().slogan,
                    models::Store::fields().created_at,
                ],
                None,
            ),
        ];

        let stores = self.aggregate_stores(pipeline, options).await?;

        let count = stores.len();

        if !pagination.need_count(count) {
            return Ok((stores, pagination.calculate_total(count)));
        }

        Ok((stores, self.count_stores(None, None).await?))
    }

    async fn get_store_for_extarnel(
        &self,
        store_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>> {
        let filter = doc! {
            "_id": store_id,
        };

        let pipeline = [
            aggregations::match_query(&filter),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    models::Store::fields().created_at,
                    models::Store::fields().updated_at,
                    models::Store::fields().name,
                    models::Store::fields().slogan,
                    models::Store::fields().description,
                    models::Store::fields().banner(true).path,
                    models::Store::fields().banner(true).file_name,
                    models::Store::fields().banner(true).mime_type,
                    models::Store::fields().banner(true).file_type,
                    models::Store::fields().logo(true).path,
                    models::Store::fields().logo(true).file_name,
                    models::Store::fields().logo(true).mime_type,
                    models::Store::fields().logo(true).file_type,
                    models::Store::fields().analytics(true).views,
                    models::Store::fields().analytics(true).rating(true).average,
                    models::Store::fields().locations,
                ],
                None,
            ),
        ];

        let store = self.aggregate_stores(pipeline, options).await?;

        let store = store.get(0).map(|s| s.to_owned());

        Ok(store)
    }
}

#[async_trait]
impl AdminStoreFunctions for DBConection {
    async fn get_stores_for_admins(
        &self,
        pagination: Option<Pagination>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let pipeline = [
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    models::Store::fields().name,
                    models::Store::fields().created_at,
                    models::Store::fields().analytics,
                    models::Store::fields().contact,
                ],
                None,
            ),
        ];

        let stores = self.aggregate_stores(pipeline, options).await?;

        let count = stores.len();

        if !pagination.need_count(count) {
            return Ok((stores, pagination.calculate_total(count)));
        }

        Ok((stores, self.count_stores(None, None).await?))
    }
}
