use crate::helpers::env::ENV_VARS;
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
        token: String,
        username: String,
        mut store_logo: String,
        store_name: String,
    ) -> ShoppaMailBuilder {
        let builder = self.build_mail(None, "");

        let mut args = HashMap::new();

        if !store_logo.starts_with(&ENV_VARS.ASSETS_URL) {
            store_logo = format!("{}{}", ENV_VARS.ASSETS_URL, store_logo);
        }

        args.insert("reg_link".to_string(), format!("{}?token={}", ENV_VARS.STORE_PANEL_URL, token));
        args.insert("username".to_string(), username);
        args.insert("store_logo".to_string(), store_logo);
        args.insert("store_name".to_string(), store_name);

        builder
            .set_template_id(ENV_VARS.NEW_STORE_USER_TEMPLATE_ID.clone())
            .set_template_args(args)
    }
}
