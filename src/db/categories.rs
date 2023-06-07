use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::options::FindOneOptions;
use shoppa_core::db::{
    models::{Categories, EmbeddedDocument, InnerCategories, InnerInnerCategories},
    DBConection,
};

#[async_trait]
pub trait CategoriesFunctions {

}

#[async_trait]
impl CategoriesFunctions for DBConection {

}
