use super::common as aggregations;
use bson::{doc, Document};

pub fn add_score_meta() -> Document {
    aggregations::add_fields(doc! {
        "score": {
            "$meta": "searchScore"
        }
    })
}
