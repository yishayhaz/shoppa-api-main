use chrono::{DateTime, Utc};

pub trait DBModel {
    fn get_collection_name() -> &'static str;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
}