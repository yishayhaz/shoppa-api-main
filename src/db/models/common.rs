use chrono::{DateTime, Utc};
use bson::oid::ObjectId;
use axum::response::Response;
pub trait DBModel {
    fn get_collection_name() -> &'static str;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn id(&self) -> Result<&ObjectId, Response>;
    fn update_id(&mut self, id: ObjectId) -> ();
}