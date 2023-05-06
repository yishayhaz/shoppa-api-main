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

    let max_edits = if query_len < 5 { 1 } else { 2 };

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
                doc! {
                    "text": {
                        "query": query,
                        "path": "name",
                        "fuzzy": {
                            "maxEdits": 2
                        },
                        "score": {
                            "boost": {
                                "value": 20
                            }
                        }
                    }
                },
                doc! {
                    "text": {
                        "query": query,
                        "path": "items.name",
                        "fuzzy": {
                            "maxEdits": 2
                        }
                    }
                },
                doc! {
                    "text": {
                        "query": query,
                        "path": "keywords",
                        "fuzzy": {
                            "maxEdits": 1
                        },
                        "score": {
                            "boost": {
                                "value": 10
                            }
                        }
                    }
                },
                doc! {
                    "text": {
                        "query": query,
                        "path": "description",
                        "score": {
                            "boost": {
                                "value": 10
                            }
                        }
                    }
                },
                doc! {
                    "text": {
                        "query": query,
                        "path": "brand",
                        "fuzzy": {
                            "maxEdits": 2
                        },
                        "score": {
                            "boost": {
                                "value": 5
                            }
                        }
                    }
                },
                doc! {
                    "text": {
                        "query": query,
                        "path": "categories.name",
                        "score": {
                            "boost": {
                                "value": 5
                            }
                        }
                    }
                },
                doc! {
                    "text": {
                        "query": query,
                        "path": "store.name",
                        "score": {
                            "boost": {
                                "value": 2
                            }
                        }
                    }
                },
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
