use core::panic;
use std::env;

#[allow(non_snake_case)]
pub struct EnvVariables {
    pub ENVIRONMENT: String,
    pub LOGIN_TOKEN_SECRET: String,
    pub CSRF_SECRET: String,
    pub DB_NAME: String,
    pub MONGODB_URI: String,
    pub PORT: String,
    pub HOST: String,
    pub COOKIE_DOMAIN: String,
    pub CORS_DOMAIN: String,
    pub DIGITAL_OCEAN_SPACE_KEY: String,
    pub DIGITAL_OCEAN_SPACE_SECRET: String,
    pub DIGITAL_OCEAN_SPACE_REGION: String,
    pub BUCKET_NAME: String
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
            PORT: env::var("PORT").unwrap_or_else(|_| String::from("3000")),
            HOST: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            COOKIE_DOMAIN: env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN must be set"),
            CORS_DOMAIN: env::var("CORS_DOMAIN").expect("CORS_DOMAIN must be set"),
            DIGITAL_OCEAN_SPACE_KEY: env::var("DIGITAL_OCEAN_SPACE_KEY")
                .expect("DIGITAL_OCEAN_SPACE_KEY must be set"),
            DIGITAL_OCEAN_SPACE_SECRET: env::var("DIGITAL_OCEAN_SPACE_SECRET")
                .expect("DIGITAL_OCEAN_SPACE_SECRET must be set"),
            DIGITAL_OCEAN_SPACE_REGION: env::var("DIGITAL_OCEAN_SPACE_REGION")
                .expect("DIGITAL_OCEAN_SPACE_REGION must be set"),
            BUCKET_NAME: env::var("BUCKET_NAME")
            .expect("BUCKET_NAME must be set"),
        }
    }
    pub fn validate(&self) {
        // checking if all the strings have a value
        if self.ENVIRONMENT.is_empty() {
            panic!("ENVIRONMENT must not be empty");
        } else if self.LOGIN_TOKEN_SECRET.len() != 32 {
            panic!("LOGIN_TOKEN_SECRET must be of length 32");
        } else if self.CSRF_SECRET.is_empty() {
            panic!("CSRF_SECRET must not be empty");
        } else if self.DB_NAME.is_empty() {
            panic!("DB_NAME must not be empty");
        } else if self.MONGODB_URI.is_empty() {
            panic!("MONGODB_URI must not be empty");
        } else if self.COOKIE_DOMAIN.is_empty() {
            panic!("COOKIE_DOMAIN must not be empty")
        } else if self.CORS_DOMAIN.is_empty() {
            panic!("CORS_DOMAIN must not be empty")
        } else if self.DIGITAL_OCEAN_SPACE_KEY.is_empty() {
            panic!("DIGITAL_OCEAN_SPACE_KEY must not be empty")
        } else if self.DIGITAL_OCEAN_SPACE_SECRET.is_empty() {
            panic!("DIGITAL_OCEAN_SPACE_SECRET must not be empty")
        } else if self.DIGITAL_OCEAN_SPACE_REGION.is_empty() {
            panic!("DIGITAL_OCEAN_SPACE_REGION must not be empty")
        } else if self.BUCKET_NAME.is_empty() {
            panic!("BUCKET_NAME must not be empty")
        }
        let port = &self.PORT;
        let host = &self.HOST;

        if port.parse::<u16>().is_err() {
            panic!("PORT must be a valid u16");
        };

        if host.parse::<std::net::IpAddr>().is_err() {
            panic!("HOST must be a valid IP address");
        };
    }
    pub fn is_production(&self) -> bool {
        self.ENVIRONMENT.contains("prod")
    }
}

lazy_static! {
    pub static ref ENV_VARS: EnvVariables = EnvVariables::new();
}
