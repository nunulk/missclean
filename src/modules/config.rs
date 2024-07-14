use dotenvy::{dotenv, var};

#[derive(Debug)]
pub struct AppConfig {
    pub misskey_api_url: String,
    pub misskey_access_token: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().expect("Failed to load .env.");

        let misskey_api_url = var("MISSKEY_API_URL").expect("Failed to get MISSKEY_API_URL.");
        let misskey_access_token =
            var("MISSKEY_ACCESS_TOKEN").expect("Failed to get MISSKEY_ACCESS_TOKEN.");

        AppConfig {
            misskey_api_url,
            misskey_access_token,
        }
    }
}