use std::vec;

use super::common as aggregations;
use bson::{doc, Document};

fn autocomplete(path: &str, query: &String) -> Document {
    let query_len = query.chars().collect::<Vec<char>>().len();

    if query_len < 3 {
        return doc! {
            "autocomplete": {
                "query": query,
                "path": path
            }
        };
    }

    let max_edits = if query_len < 4 { 1 } else { 2 };

    doc! {
        "autocomplete": {
            "query": query,
            "path": path,
            "fuzzy": {
                "maxEdits": max_edits,
            }
        }
    }
}

fn text_search(
    path: &str,
    query: &String,
    score_boost: Option<u8>,
    max_edits: Option<u8>,
) -> Document {
    // max_edits should be 1 or 2
    let mut base = doc! {
            "query": query,
            "path": path,
    };

    if let Some(score_boost) = score_boost {
        base.insert(
            "score",
            doc! {
                "boost": {
                    "value": score_boost as i32
                }
            },
        );
    };

    if let Some(max_edits) = max_edits {
        base.insert(
            "fuzzy",
            doc! {
                "maxEdits": max_edits as i32
            },
        );
    };

    doc! {
        "text": base
    }
}

pub fn search_products(
    query: &Option<String>,
    filters: &Vec<Document>,
    minimum_should_match: Option<i32>,
) -> Document {
    if query.is_none() && filters.is_empty() {
        return aggregations::match_query(&doc! {});
    }

    let mut compound = doc! {};

    if let Some(query) = query {
        compound.insert(
            "should",
            vec![
                text_search("name", query, Some(20), Some(2)),
                text_search("items.name", query, None, Some(2)),
                text_search("keywords", query, Some(10), Some(1)),
                text_search("description", query, Some(10), None),
                text_search("brand", query, Some(5), Some(2)),
                text_search("categories.name", query, Some(5), None),
                text_search("store.name", query, Some(2), None),
            ],
        );
        compound.insert("minimumShouldMatch", minimum_should_match.unwrap_or(1));
    }

    compound.insert("filter", filters);

    aggregations::search(doc! {
        "compound": compound
    })
}

pub fn autocomplete_products_search(query: &String, filters: Vec<Document>) -> Document {
    aggregations::search(doc! {
            "compound": {
            "should": [
                autocomplete("name", query),
                autocomplete("items.name", query),
            ],
            "filter": filters,
            "minimumShouldMatch": 1
        }
    })
}

pub fn autocomplete_store_search(query: &String) -> Document {
    aggregations::search(doc! {
        "compound": {
            "should": [
                autocomplete("name", query),
            ],
        }
    })
}
