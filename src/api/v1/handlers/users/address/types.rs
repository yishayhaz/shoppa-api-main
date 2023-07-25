use crate::prelude::types::*;

pub struct AddUserAddress {
    pub city: String,
    pub free_text: Option<String>,
    pub street: String,
    pub street_number: String,
    pub entrance: Option<String>,
    pub floor: Option<i16>,
    pub apartment: Option<String>,
    pub zip_code: String,
}

pub struct EditUserAddress {
    pub city: String,
    pub free_text: Option<String>,
    pub street: String,
    pub street_number: String,
    pub entrance: Option<String>,
    pub floor: Option<i16>,
    pub apartment: Option<String>,
    pub zip_code: String,
}
