use super::common::DBModel;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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
    pub address: Option<String>,
    pub credit_cards: Option<Vec<ObjectId>>,
    // maybe move the cart to be a struct
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