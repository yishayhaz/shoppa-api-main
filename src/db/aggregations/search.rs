use std::vec;

use super::common as aggregations;
use bson::{doc, Document};

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
            "index": "autocomplete",
            "compound": {
            "should": [
                {
                    "autocomplete": {
                        "query": query,
                        "path": "name",
                        "fuzzy": {
                            "maxEdits": 2
                        }
                    }
                },
                {
                    "autocomplete": {
                        "query": query,
                        "path": "items.name",
                        "fuzzy": {
                            "maxEdits": 2
                        }
                    }
                }
            ],
            "filter": filters,
            "minimumShouldMatch": 1
        }
    })
}
