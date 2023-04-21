use super::common as aggregations;
use bson::Document;
use crate::db::models::{Variants, DBModel, Product};

pub fn lookup_product_variants(pipeline: Option<Vec<Document>>) -> Document {
    aggregations::lookup(
        Variants::get_collection_name(), 
        Product::fields().variants, 
        Variants::fields().id, 
        Product::fields().variants, pipeline, 
        None
    )
}