use super::common::DBModel;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::helpers::types::ResponseBuilder;
use axum::response::{Response, IntoResponse};

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub level: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub gender: Option<Genders>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde_as(as = "Option<bson::DateTime>")]
    pub date_of_birth: Option<DateTime<Utc>>,
    pub address: Option<Vec<String>>,
    pub credit_cards: Option<Vec<String>>,
    pub cart: Cart,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub coupon: Option<String>,
    pub total_price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartItem {
    // TODO enum that can be product or ObjectId
    pub product_id: ObjectId,
    pub quantity: i32,
}



#[derive(Serialize, Deserialize, Debug)]
pub enum Genders {
    Male,
    Female,
}

impl DBModel for User {
    fn get_collection_name() -> &'static str {
        "users"
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    fn id(&self) -> Result<&ObjectId, Response> {
        match &self.id {
            Some(id) => Ok(id),
            None => Err(ResponseBuilder::<u16>::error(None, Some(String::from("User id is None")), Some(500)).into_response()),
        }
    }
    fn update_id(&mut self, id: ObjectId) -> () {

        match self.id {
            Some(_) => return (),
            None => (),
        }

        self.id = Some(id);
    }
}

impl User {
    pub fn new() -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            level: 1,
            name: None,
            email: None,
            phone_number: None,
            password: None,
            gender: None,
            date_of_birth: None,
            address: None,
            credit_cards: None,
            cart: Cart::new(),
        }
    }
}

impl Cart {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            coupon: None,
            total_price: 0.0,
        }
    }
}