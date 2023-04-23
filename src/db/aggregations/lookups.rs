use super::common as aggregations;
use crate::db::models::{DBModel, Product, Store, Variants};
use bson::Document;

pub fn lookup_product_variants(pipeline: Option<Vec<Document>>) -> Document {
    aggregations::lookup(
        Variants::get_collection_name(),
        Product::fields().variants,
        Variants::fields().id,
        Product::fields().variants,
        pipeline,
        None,
    )
}

pub fn lookup_product_shop(pipeline: Option<Vec<Document>>) -> [Document; 2] {
    [
        aggregations::lookup(
            Store::get_collection_name(),
            format!(
                "{}.{}",
                Product::fields().store,
                Product::fields().store().id
            )
            .as_str(),
            Store::fields().id,
            Product::fields().store,
            pipeline,
            None,
        ),
        aggregations::unwind(Product::fields().store, false),
    ]
}
