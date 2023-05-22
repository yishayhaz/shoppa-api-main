use super::common as aggregations;
use bson::{doc, Document};

pub fn sort_by_score() -> Document {
    aggregations::sort(doc! {
        "score": -1
    })
}
