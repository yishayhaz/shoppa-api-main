use super::super::models::DBModel;
use bson::{doc, Document};

pub enum ProjectIdOptions {
    Keep,
    Remove,
    ToString,
}

pub fn lookup<T: DBModel>(
    _collection: T,
    local_field: &'static str,
    foreign_field: &'static str,
    as_: &'static str,
    pipeline: Option<Vec<Document>>,
    let_: Option<Document>,
) -> Document {
    doc! {
        "$lookup": {
            "from":  T::get_collection_name(),
            "localField": local_field,
            "foreignField": foreign_field,
            "as": as_,
            "let": let_,
            "pipeline": pipeline,
        }
    }
}

pub fn unwind(path: &'static str, preserve: bool) -> Document {
    doc! {
        "$unwind": {
            "path": format!("${}", path),
            "preserveNullAndEmptyArrays": preserve,
        }
    }
}

pub fn match_query(query: Document) -> Document {
    doc! {
        "$match": query,
    }
}

pub fn unset(fields: Vec<&'static str>) -> Document {
    doc! {
        "$unset": fields
    }
}

pub fn project(
    id_option: ProjectIdOptions,
    keep: Vec<&'static str>,
    extra: Option<Document>,
) -> Document {
    let mut project_stage = Document::new();

    for key in keep {
        project_stage.insert(key, 1);
    }

    match id_option {
        ProjectIdOptions::Keep => {}
        ProjectIdOptions::Remove => {
            project_stage.insert("_id", 0);
        }
        ProjectIdOptions::ToString => {
            project_stage.insert("_id", doc! {"$toString": "$_id"});
        }
    }

    match extra {
        Some(d) => project_stage.extend(d),
        None => {}
    }

    doc! {
        "$project": project_stage
    }
}

pub fn add_fields(fields: Document) -> Document {
    doc! {
        "$addFields": fields
    }
}

pub fn skip(count: i64) -> Document {
    doc! {
        "$skip": count,
    }
}

pub fn limit(count: i64) -> Document {
    doc! {
        "$limit": count,
    }
}

pub fn sort(fields: Document) -> Document {
    doc! {
        "$sort": fields,
    }
}

pub fn count(key: &'static str) -> Document {
    doc! {
        "$count": key,
    }
}

pub fn group(fields: Document) -> Document {
    doc! {
        "$group": fields
    }
}

pub fn replace_root(new_root: &'static str) -> Document {
    doc! {
        "$replaceRoot": {
            "newRoot": format!("${}", new_root),
        }
    }
}

pub fn facet(fields: Document) -> Document {
    doc! {
        "$facet": fields,
    }
}

pub fn convert_to_string_safe(field: &'static str) -> Document {
    doc! {
        "$convert": {
            "input": field,
            "to": "string",
            "onError": "error",
            "onNull": "null",
        }
    }
}

pub fn safe_array_size(field: &'static str) -> Document {
    doc! {
        "$cond": [
            {"$isArray": field},
            {"$size": field},
            0,
        ]
    }
}

pub fn union_with<T: DBModel>(_collection: T, pipeline: Document) -> Document {
    doc! {
        "$unionWith": {
            "coll": T::get_collection_name(),
            "pipeline": pipeline,
        }
    }
}
