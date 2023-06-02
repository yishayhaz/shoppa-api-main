use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{AggregateOptions, FindOneAndUpdateOptions};
use shoppa_core::{
    constans,
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{self, EmbeddedDocument, FileDocument, Product, ProductItem},
        DBConection, Pagination,
    },
};

#[async_trait]
pub trait ProductFunctions {
    async fn add_view_to_product(
        &self,
        product_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;
}

#[async_trait]
pub trait AdminProductFunctions {
    async fn add_product_item(
        &self,
        product_id: &ObjectId,
        item: &ProductItem,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn add_asset_to_product(
        &self,
        product_id: &ObjectId,
        asset: &FileDocument,
        items_ids: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;

    async fn edit_product_item(
        &self,
        product_id: &ObjectId,
        item_id: &ObjectId,
        price: Option<f64>,
        in_storage: Option<u64>,
        name: Option<String>,
        images_refs: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>>;
}

#[async_trait]
impl ProductFunctions for DBConection {
    async fn add_view_to_product(
        &self,
        product_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let update = doc! {
            "$inc": {
                Product::fields().analytics(true).views: 1
            }
        };

        self.find_and_update_product_by_id(product_id, update, options)
            .await
    }
}

#[async_trait]
impl AdminProductFunctions for DBConection {
    async fn add_product_item(
        &self,
        product_id: &ObjectId,
        item: &ProductItem,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let update = doc! {
            "$push": {
                "items": item.into_bson()?
            }
        };

        self.find_and_update_product_by_id(product_id, update, options)
            .await
    }

    async fn add_asset_to_product(
        &self,
        product_id: &ObjectId,
        asset: &FileDocument,
        items_ids: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {
        let mut options = options.unwrap_or_default();
        // the update push operation
        let mut push = doc! {
            Product::fields().assets: asset.into_bson()?
        };
        // if items ids are provided, then we need to push the asset to the items refs
        if let Some(items_ids) = items_ids {
            // adding the asset to the items refs using the array filter
            push.insert(
                format!(
                    "{}.$[item].{}",
                    Product::fields().items,
                    Product::fields().items(false).assets_refs
                ),
                asset.into_bson()?,
            );

            let item_filter = doc! {
                "item": {
                    Product::fields().items(true).id: {
                        "$in": items_ids
                    }
                }
            };

            // adding the array filter to the options, if there is already an array filter
            let mut array_filter = options.array_filters.unwrap_or_default();

            array_filter.push(item_filter);

            options.array_filters = Some(array_filter);
        }

        let update = doc! {
            "$push": push
        };

        self.find_and_update_product_by_id(product_id, update, options.into())
            .await
    }

    async fn edit_product_item(
        &self,
        product_id: &ObjectId,
        item_id: &ObjectId,
        price: Option<f64>,
        in_storage: Option<u64>,
        name: Option<String>,
        assets_refs: Option<Vec<ObjectId>>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Product>> {

        let filters = doc! {
            Product::fields().id: product_id,
            Product::fields().items(true).id: item_id
        };

        let mut update = doc! {};

        if let Some(price) = price {
            let field = format!("{}.$.{}", Product::fields().items, Product::fields().items(false).price);
            update.insert(field, price);
        }

        if let Some(in_storage) = in_storage {
            let field = format!("{}.$.{}", Product::fields().items, Product::fields().items(false).in_storage);
            update.insert(field, in_storage as i64);
        }

        if let Some(name) = name {
            let field = format!("{}.$.{}", Product::fields().items, Product::fields().items(false).name);
            if name == constans::DELETE_FIELD_KEY_OPETATOR {
                update.insert::<_, Option<String>>(field, None);
            } else {
                update.insert(field, name);
            }
        }

        if let Some(assets_refs) = assets_refs {
            let field = format!("{}.$.{}", Product::fields().items, Product::fields().items(false).assets_refs);
            update.insert(field, assets_refs);
        }

        let update_at_field = format!("{}.$.{}", Product::fields().items, Product::fields().items(false).updated_at);

        update.insert(update_at_field, chrono::Utc::now());

        let update = doc! {
            "$set": update
        };

        self.find_and_update_product(filters, update, options)
            .await
    }
}
