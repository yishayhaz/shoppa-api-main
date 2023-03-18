use std::env;

#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum EnvVars {
    ENVIRONMENT,
    LOGIN_TOKEN_SECRET,
    CSRF_SECRET,
    DB_NAME,
    MONGODB_URI,
    PORT,
    HOST,
    COOKIE_DOMAIN,
}

impl EnvVars {
    pub fn get(&self) -> String {
        match self {
            Self::ENVIRONMENT => {
                env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
            }
            Self::LOGIN_TOKEN_SECRET => {
                env::var("LOGIN_TOKEN_SECRET").expect("LOGIN_TOKEN_SECRET must be set")
            }
            Self::CSRF_SECRET => env::var("CSRF_SECRET").expect("CSRF_SECRET must be set"),
            Self::DB_NAME => env::var("DB_NAME").expect("DB_NAME must be set"),
            Self::MONGODB_URI => env::var("MONGODB_URI").expect("MONGODB_URI must be set"),
            Self::PORT => env::var("PORT").unwrap_or_else(|_| String::from("3000")),
            Self::HOST => env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            Self::COOKIE_DOMAIN => env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN must be set"),
        }
    }

    pub fn validate() {
        // checking if all the strings have a value
        if Self::ENVIRONMENT.get().is_empty() {
            panic!("ENVIRONMENT must not be empty");
        } else if Self::LOGIN_TOKEN_SECRET.get().is_empty() {
            panic!("LOGIN_TOKEN_SECRET must not be empty");
        } else if Self::CSRF_SECRET.get().is_empty() {
            panic!("CSRF_SECRET must not be empty");
        } else if Self::DB_NAME.get().is_empty() {
            panic!("DB_NAME must not be empty");
        } else if Self::MONGODB_URI.get().is_empty() {
            panic!("MONGODB_URI must not be empty");
        } else if Self::COOKIE_DOMAIN.get().is_empty() {
            panic!("COOKIE_DOMAIN must not be empty")
        }
        let port = Self::PORT.get();
        let host = Self::HOST.get();

        if port.parse::<u16>().is_err() {
            panic!("PORT must be a valid u16");
        };

        if host.parse::<std::net::IpAddr>().is_err() {
            panic!("HOST must be a valid IP address");
        };
    }

    pub fn is_production() -> bool {
        Self::ENVIRONMENT.get().contains("prod")
    }
}
