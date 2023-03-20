use chrono::{DateTime, Utc};
use bson::oid::ObjectId;
use axum::response::Response;
use mongodb::IndexModel;

pub trait DBModel {
    fn get_collection_name() -> &'static str;
    fn get_indexes() -> Vec<IndexModel>;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn id(&self) -> Result<&ObjectId, Response>;
    fn update_id(&mut self, id: ObjectId) -> ();
}

macro_rules! db_model {
    ($Struct:ty) => {
        fn created_at(&self) -> DateTime<Utc> {
            self.created_at
        }

        fn updated_at(&self) -> DateTime<Utc> {
            self.updated_at
        }

        fn id(&self) -> Result<&ObjectId, Response> {
            match &self.id {
                Some(id) => Ok(id),
                // TODO add error code here
                None => Err(ResponseBuilder::<u16>::error("", None, Some(concat!(stringify!($Self), " id is None")), Some(500)).into_response()),
            }
        }

        fn update_id(&mut self, id: ObjectId) -> () {
            match self.id {
                Some(_) => return (),
                None => (),
            }

            self.id = Some(id);
        }
    };
}

pub(crate) use db_model;
