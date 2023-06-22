use shoppa_core::email_sender::{EmailClient, ShoppaMailBuilder};
use std::collections::HashMap;

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
    ) -> ShoppaMailBuilder {
        let builder = self.build_mail(None, "ברוכים הבאים לשופ״ה");

        let mut args = HashMap::new();

        args.insert("reg_link".to_string(), reg_link);
        args.insert("username".to_string(), username);
        args.insert("store_logo".to_string(), store_logo);
        args.insert("store_name".to_string(), store_name);

        builder
            .set_template_id("0e823a84-62b8-4841-a36c-c6d005c7f6e7".to_string())
            .set_template_args(args)
    }
}
