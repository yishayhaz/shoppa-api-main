use crate::{prelude::types::*, db::models::NewsLetterSubscriber};

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