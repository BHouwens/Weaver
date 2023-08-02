use crate::constants::{
    CONFIG_FILE, DRUID_CHARSET, DRUID_LENGTH, SETTINGS_CACHE_PASSWORD, SETTINGS_CACHE_PORT,
    SETTINGS_CACHE_URL, SETTINGS_DB_PASSWORD, SETTINGS_DB_PORT, SETTINGS_DB_URL, SETTINGS_DEBUG,
    SETTINGS_EXTERN_PORT,
};
use crate::crypto::sign_ed25519 as sign;
use crate::crypto::sign_ed25519::{PublicKey, Signature};
use crate::interfaces::EnvConfig;
use chrono::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Loads the config file
pub fn load_config() -> EnvConfig {
    let settings = config::Config::builder().add_source(config::File::with_name(CONFIG_FILE));

    match settings.build() {
        Ok(config) => EnvConfig {
            debug: config.get_bool("debug").unwrap_or(SETTINGS_DEBUG),
            extern_port: config
                .get_string("extern_port")
                .unwrap_or(SETTINGS_EXTERN_PORT.to_string()),
            db_url: config
                .get_string("db_url")
                .unwrap_or(SETTINGS_DB_URL.to_string()),
            db_port: config
                .get_string("db_port")
                .unwrap_or(SETTINGS_DB_PORT.to_string()),
            db_password: config
                .get_string("db_password")
                .unwrap_or(SETTINGS_DB_PASSWORD.to_string()),
            cache_url: config
                .get_string("cache_url")
                .unwrap_or(SETTINGS_CACHE_URL.to_string()),
            cache_port: config
                .get_string("cache_port")
                .unwrap_or(SETTINGS_CACHE_PORT.to_string()),
            cache_password: config
                .get_string("cache_password")
                .unwrap_or(SETTINGS_CACHE_PASSWORD.to_string()),
        },
        Err(e) => {
            panic!("Failed to load config file with error: {}", e)
        }
    }
}

/// Function to validate the signature using Ed25519
pub fn validate_signature(public_key: &str, msg: &str, signature: &str) -> bool {
    let pk = PublicKey::from_slice(public_key.as_bytes()).unwrap();
    let signature = Signature::from_slice(signature.as_bytes()).unwrap();

    sign::verify_detached(&signature, msg.as_bytes(), &pk)
}

/// Function to serialize data
pub fn serialize_data<T: Serialize + for<'a> Deserialize<'a>>(data: T) -> String {
    serde_json::to_string(&data).unwrap()
}

/// Function to deserialize data
pub fn deserialize_data<T: Serialize + for<'a> Deserialize<'a>>(data: String) -> T {
    serde_json::from_str(&data).unwrap()
}

/// Constructs a 16 byte DRUID string
pub fn construct_druid() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..DRUID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..DRUID_CHARSET.len());
            DRUID_CHARSET[idx] as char
        })
        .collect();

    random_string
}

/// Constructs a string-formatted date
pub fn construct_formatted_date() -> String {
    let utc_now: DateTime<Utc> = Utc::now();
    utc_now.format("%Y-%m-%d %H:%M:%S").to_string()
}
