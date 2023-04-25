use crate::prelude::types::*;
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignUpToNewsLetterPayload {
    #[validate(email)]
    pub email: String,
}
