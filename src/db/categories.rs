use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneOptions, FindOptions};
use shoppa_core::db::{
    models::Category,
    DBConection,
    populate::CategoriesPopulate
};

#[async_trait]
pub trait CategoriesFunctions {
}

#[async_trait]
impl CategoriesFunctions for DBConection {

}
