use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{AggregateOptions, FindOneAndUpdateOptions};
use shoppa_core::{
    constans,
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{self, EmbeddedDocument, Product, ProductItem},
        DBConection, Pagination,
    },
};

#[async_trait]
pub trait ProductFunctions {
    async fn add_view_to_product(
        &self,
        product_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Document>>;
}

#[async_trait]
pub trait AdminProductFunctions {}

#[async_trait]
impl ProductFunctions for DBConection {
    async fn add_view_to_product(
        &self,
        product_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Document>> {
        let update = doc! {
            "$inc": {
                Product::fields().analytics(true).views: 1
            }
        };

        self.find_and_update_product_by_id(product_id, update, options)
    }
}
