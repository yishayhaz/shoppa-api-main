use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignUpToNewsLetterPayload {
    #[validate(email)]
    pub email: String,
}
