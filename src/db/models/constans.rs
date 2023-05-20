// Store related constants
pub const STORE_NAME_MIN_LENGTH: usize = 3;
pub const STORE_NAME_MAX_LENGTH: usize = 60;
pub const STORE_DESCRIPTION_MIN_LENGTH: usize = 20;
pub const STORE_DESCRIPTION_MAX_LENGTH: usize = 160;
pub const STORE_SLOGAN_MIN_LENGTH: usize = 8;
pub const STORE_SLOGAN_MAX_LENGTH: usize = 40;

// General constants
pub const CITY_NAME_MIN_LENGTH: usize = 2;
pub const CITY_NAME_MAX_LENGTH: usize = 85;
pub const STREET_NAME_MIN_LENGTH: usize = 2;
pub const STREET_NAME_MAX_LENGTH: usize = 85;
pub const STREET_NUMBER_MIN_LENGTH: usize = 2;
pub const STREET_NUMBER_MAX_LENGTH: usize = 85;
pub const LOCATION_FREE_TEXT_MAX_LENGTH: usize = 60;

// Simple email regex to validate that there is at least one @ and one .
pub const EMAIL_REGEX: &str = r"^(?=.*@)(?=.*\.)";
// Simple phone regex to validate that there is at least one + and 7-17 digits
pub const PHONE_REGEX: &str = r"^\+\d{7,17}$";
pub const NUMBER_STRING_REGEX: &str = r"^\d+$";

pub const DELETE_FIELD_KEY_OPETATOR: &str = "$delete_this$";
