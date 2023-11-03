use once_cell::sync::Lazy;

pub struct Config {
    pub jwt_secret: String,
    pub salt: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            salt: std::env::var("SALT").expect("SALT must be set"),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new());
