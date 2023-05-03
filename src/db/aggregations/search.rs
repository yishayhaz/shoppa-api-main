use std::vec;

use super::common as aggregations;
use bson::{doc, oid::ObjectId, Document};

pub fn search_products(
    query: &Option<String>,
    category_id: &Option<ObjectId>,
    store_id: &Option<ObjectId>,
) -> Vec<Document> {
    if query.is_none() && category_id.is_none() && store_id.is_none() {
        return vec![aggregations::match_query(&doc! {})];
    }

    let mut filters = vec![];

    if let Some(category_id) = category_id {
        filters.push(doc! {
            "equals": {
                "value": category_id,
                "path": "categories._id"
            }
        });
    }

    if let Some(store_id) = store_id {
        filters.push(doc! {
            "equals": {
                "value": store_id,
                "path": "store._id"
            }
        });
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
        compound.insert("minimumShouldMatch", 1);
    }

    if filters.len() > 0 {
        compound.insert("filter", filters);
    }

    vec![
        aggregations::search(doc! {
            "compound": compound
        }),
        aggregations::add_fields(doc! {
                "score": {
                    "$meta": "searchScore"
                }

        }),
        aggregations::sort(doc! {
            "score": -1
        }),
    ]
}

pub fn autocomplete_products_search(query: &String) -> Document {
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
            "minimumShouldMatch": 1
        }
    })
}
