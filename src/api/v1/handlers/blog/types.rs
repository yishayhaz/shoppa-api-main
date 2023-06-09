use crate::prelude::types::*;
use shoppa_core::db::models::NewsLetterSubscriber;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignUpToNewsLetterPayload {
    #[validate(email)]
    pub email: String,
}

impl Into<NewsLetterSubscriber> for SignUpToNewsLetterPayload {
    fn into(self) -> NewsLetterSubscriber {
        NewsLetterSubscriber::new(self.email)
    }
}
