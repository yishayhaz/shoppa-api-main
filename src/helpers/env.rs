use std::env;
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Debug, Validate)]
pub struct EnvVariables {
    #[validate(length(min = 1))]
    pub ENVIRONMENT: String,
    #[validate(length(equal = 32))]
    pub LOGIN_TOKEN_SECRET: String,
    #[validate(length(min = 1))]
    pub CSRF_SECRET: String,
    #[validate(length(min = 1))]
    pub DB_NAME: String,
    #[validate(length(min = 1))]
    pub MONGODB_URI: String,
    pub PORT: u16,
    pub HOST: std::net::IpAddr,
    #[validate(length(min = 1))]
    pub COOKIE_DOMAIN: String,
    #[validate(length(min = 1))]
    pub CORS_DOMAIN: String,
    #[validate(length(min = 1))]
    pub DIGITAL_OCEAN_SPACE_KEY: String,
    #[validate(length(min = 1))]
    pub DIGITAL_OCEAN_SPACE_SECRET: String,
    #[validate(length(min = 1))]
    pub DIGITAL_OCEAN_SPACE_REGION: String,
    #[validate(length(min = 1))]
    pub BUCKET_NAME: String,
}

impl EnvVariables {
    pub fn new() -> Self {
        Self {
            ENVIRONMENT: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            LOGIN_TOKEN_SECRET: env::var("LOGIN_TOKEN_SECRET")
                .expect("LOGIN_TOKEN_SECRET must be set"),
            CSRF_SECRET: env::var("CSRF_SECRET").expect("CSRF_SECRET must be set"),
            DB_NAME: env::var("DB_NAME").expect("DB_NAME must be set"),
            MONGODB_URI: env::var("MONGODB_URI").expect("MONGODB_URI must be set"),
            PORT: env::var("PORT")
                .unwrap_or_else(|_| String::from("8000"))
                .parse()
                .expect("PORT must be a valid u16"),
            HOST: env::var("HOST")
                .unwrap_or_else(|_| String::from("127.0.0.1"))
                .parse()
                .expect("HOST must be a valid IP address"),
            COOKIE_DOMAIN: env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN must be set"),
            CORS_DOMAIN: env::var("CORS_DOMAIN").expect("CORS_DOMAIN must be set"),
            DIGITAL_OCEAN_SPACE_KEY: env::var("DIGITAL_OCEAN_SPACE_KEY")
                .expect("DIGITAL_OCEAN_SPACE_KEY must be set"),
            DIGITAL_OCEAN_SPACE_SECRET: env::var("DIGITAL_OCEAN_SPACE_SECRET")
                .expect("DIGITAL_OCEAN_SPACE_SECRET must be set"),
            DIGITAL_OCEAN_SPACE_REGION: env::var("DIGITAL_OCEAN_SPACE_REGION")
                .expect("DIGITAL_OCEAN_SPACE_REGION must be set"),
            BUCKET_NAME: env::var("BUCKET_NAME").expect("BUCKET_NAME must be set"),
        }
    }
    pub fn is_production(&self) -> bool {
        self.ENVIRONMENT.contains("prod")
    }
}

lazy_static! {
    pub static ref ENV_VARS: EnvVariables = EnvVariables::new();
}
