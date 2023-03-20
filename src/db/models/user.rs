use std::io::Seek;

use super::common::{db_model, DBModel};
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

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
    pub address: Vec<Address>,
    pub credit_cards: Vec<CreditCard>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub id: String,
    pub name: String,
    pub default: bool,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCard {
    pub id: String,
    pub name: String,
    pub default: bool,
    pub deleted: bool,
}

impl DBModel for User {
    fn get_collection_name() -> &'static str {
        "users"
    }

    fn get_indexes() -> Vec<IndexModel> {
        vec![]
    }

    db_model!(User);
}

impl User {
    pub fn new(
        email: Option<String>,
        password: Option<String>,
        name: Option<String>,
        level: i32,
    ) -> Self {
        if level > 3 {
            panic!("Level must be lower then 3")
        };

        if level > 1 {
            if password.is_none() {
                panic!("If level is greater then 1 you must provide a passowrd")
            } else if email.is_none() {
                panic!("If level is greater then 1 you must provide a email")
            } else if name.is_none() {
                panic!("If level is greater then 1 you must provide a username")
            }
        };

        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            level: 1,
            name: name,
            email: email,
            phone_number: None,
            password: password,
            gender: None,
            date_of_birth: None,
            address: vec![],
            credit_cards: vec![],
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
