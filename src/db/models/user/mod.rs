mod fields;

use super::{
    common::{db_model, DBModel, RefrenceField},
    Product,
};
use serde_json::{json, Value};
use crate::prelude::{db_models::*, *};

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub coupon: Option<String>,
    pub total_price: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartItem {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub added_at: DateTime<Utc>,
    pub product: RefrenceField<Product, ObjectId>,
    pub item_id: ObjectId,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Genders {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub id: String,
    pub name: String,
    pub default: bool,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            level,
            name,
            email,
            phone_number: None,
            password,
            gender: None,
            date_of_birth: None,
            address: vec![],
            credit_cards: vec![],
            cart: Cart::new(),
        }
    }

    fn date_of_birth_as_string(&self) -> Option<String> {
        match self.date_of_birth {
            Some(date) => Some(date.to_string()),
            None => None,
        }
    }

    pub fn to_get_me(&self) -> Result<Value> {
        Ok(json!({
            "_id": self.id()?.to_string(),
            "name": self.name,
            "cart": self.cart,
            "gender": self.gender,
            "phone_number": self.phone_number,
            "level": self.level,
            "address": self.address,
            "email": self.email,
            "created_at": self.created_at().to_string(),
            "date_of_birth": self.date_of_birth_as_string()
        }))
    }

    pub fn fields() -> &'static fields::UsersFields {
        &fields::FIELDS
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
