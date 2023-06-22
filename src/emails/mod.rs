use shoppa_core::email_sender::{ShoppaMailBuilder, EmailClient};

pub trait AdminEmailFunctions {
    fn new_store_user_email(
        &self,
        reg_link: String,
        username: String,
        store_logo: String,
        store_name: String,
    ) -> ShoppaMailBuilder;
}

impl AdminEmailFunctions for EmailClient {
    fn new_store_user_email(
        &self,
        reg_link: String,
        username: String,
        store_logo: String,
        store_name: String,
    ) -> ShoppaMailBuilder{
        let mut builder = self::build_mail(None, "ברוכים הבאים לשופה");
    }
}

