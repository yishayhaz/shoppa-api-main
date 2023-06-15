use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use shoppa_core::db::{models::Variants, DBConection};

#[async_trait]
pub trait AdminVariantsFunctions {
    async fn validate_variants_exist(&self, ids: &Vec<ObjectId>) -> Result<bool>;
    async fn get_variants_by_ids(&self, ids: &Vec<ObjectId>) -> Result<Vec<Variants>>;
}

#[async_trait]
impl AdminVariantsFunctions for DBConection {
    async fn validate_variants_exist(&self, ids: &Vec<ObjectId>) -> Result<bool> {
        if ids.is_empty() {
            return Ok(true);
        }

        let count = self
            .count_variants(
                Some(doc! {
                    Variants::fields().id: {
                        "$in": ids
                    }
                }),
                None,
                None,
            )
            .await?;

        Ok(count == ids.len() as u64)
    }

    async fn get_variants_by_ids(&self, ids: &Vec<ObjectId>) -> Result<Vec<Variants>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        self.get_variants(
            doc! {
                Variants::fields().id: {
                    "$in": ids
                }
            },
            None,
            None,
            None,
        )
        .await
    }
}
