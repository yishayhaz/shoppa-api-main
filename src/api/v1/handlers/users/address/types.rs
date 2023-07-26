use crate::prelude::types::*;
use bson::Document;
use shoppa_core::{
    constans, db::models::Address, parser::FieldPatch, validators::number_string_validator,
};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AddUserAddress {
    pub free_text: Option<String>,
    #[validate(length(
        min = "constans::CITY_NAME_MIN_LENGTH",
        max = "constans::CITY_NAME_MAX_LENGTH"
    ))]
    pub city: String,
    #[validate(length(
        min = "constans::STREET_NAME_MIN_LENGTH",
        max = "constans::STREET_NAME_MAX_LENGTH"
    ))]
    pub street: String,
    #[validate(length(
        min = "constans::STREET_NUMBER_MIN_LENGTH",
        max = "constans::STREET_NUMBER_MAX_LENGTH"
    ))]
    pub street_number: String,
    #[validate(length(
        min = "constans::ENTRANCE_MIN_LENGTH",
        max = "constans::ENTRANCE_MAX_LENGTH"
    ))]
    pub entrance: Option<String>,
    #[validate(range(min = "constans::FLOOR_MIN", max = "constans::FLOOR_MAX"))]
    pub floor: Option<i16>,
    #[validate(length(
        min = "constans::APARTMENT_MIN_LENGTH",
        max = "constans::APARTMENT_MAX_LENGTH"
    ))]
    pub apartment: Option<String>,
    #[validate(
        custom = "number_string_validator",
        length(min = "constans::ZIP_MIN_LENGTH", max = "constans::ZIP_MAX_LENGTH")
    )]
    pub zip_code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EditUserAddress {
    #[serde(default)]
    pub free_text: FieldPatch<String>,
    #[validate(length(
        min = "constans::CITY_NAME_MIN_LENGTH",
        max = "constans::CITY_NAME_MAX_LENGTH"
    ))]
    pub city: Option<String>,
    #[validate(length(
        min = "constans::STREET_NAME_MIN_LENGTH",
        max = "constans::STREET_NAME_MAX_LENGTH"
    ))]
    pub street: Option<String>,
    #[validate(length(
        min = "constans::STREET_NUMBER_MIN_LENGTH",
        max = "constans::STREET_NUMBER_MAX_LENGTH"
    ))]
    pub street_number: Option<String>,
    #[serde(default)]
    // #[validate(length(
    //     min = "constans::ENTRANCE_MIN_LENGTH",
    //     max = "constans::ENTRANCE_MAX_LENGTH"
    // ))]
    pub entrance: FieldPatch<String>,
    // #[validate(range(min = "constans::FLOOR_MIN", max = "constans::FLOOR_MAX"))]
    #[serde(default)]
    pub floor: FieldPatch<i16>,
    #[serde(default)]
    #[validate(length(
        min = "constans::APARTMENT_MIN_LENGTH",
        max = "constans::APARTMENT_MAX_LENGTH"
    ))]
    pub apartment: FieldPatch<String>,
    #[validate(
        custom = "number_string_validator",
        length(min = "constans::ZIP_MIN_LENGTH", max = "constans::ZIP_MAX_LENGTH")
    )]
    pub zip_code: Option<String>,
}

impl Into<Address> for AddUserAddress {
    fn into(self) -> Address {
        Address::new(
            self.city,
            self.street,
            self.street_number,
            self.entrance,
            self.floor,
            self.apartment,
            self.zip_code,
        )
    }
}

impl Into<Document> for EditUserAddress {
    fn into(self) -> Document {
        let mut doc = Document::new();

        if self.free_text != FieldPatch::Missing {
            doc.insert(Address::fields().free_text, self.free_text.into_option());
        }

        if let Some(city) = self.city {
            doc.insert(Address::fields().city, city);
        }

        if let Some(street) = self.street {
            doc.insert(Address::fields().street, street);
        }

        if let Some(street_number) = self.street_number {
            doc.insert(Address::fields().street_number, street_number);
        }

        if self.entrance != FieldPatch::Missing {
            doc.insert(Address::fields().entrance, self.entrance.into_option());
        }

        if self.floor != FieldPatch::Missing {
            doc.insert(
                Address::fields().floor,
                self.floor.into_option().map(|x| {
                    if let Some(x) = x {
                        Some(x as i32)
                    } else {
                        None
                    }
                }),
            );
        }

        if self.apartment != FieldPatch::Missing {
            doc.insert(Address::fields().apartment, self.apartment.into_option());
        }

        if let Some(zip_code) = self.zip_code {
            doc.insert(Address::fields().zip_code, zip_code);
        }

        doc
    }
}
