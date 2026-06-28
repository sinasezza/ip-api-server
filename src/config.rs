use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub server_host: String,
    
    // Added missing fields from .env
    pub api_timeout: u64,
    pub max_requests_per_minute: u32,
    
    pub api_secret_key: String,
    pub admin_token: String,
    pub rust_env: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the .env file (if it exists), ignore errors if it doesn't
        dotenvy::dotenv().ok();

        Ok(Config {
            // We use unwrap_or_else to provide sensible defaults for optional settings
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            
            // Parse the newly added fields with sensible defaults
            api_timeout: std::env::var("API_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            max_requests_per_minute: std::env::var("MAX_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
            
            // These are required, so we use `?` to propagate the error if missing
            api_secret_key: std::env::var("API_SECRET_KEY")?,
            admin_token: std::env::var("ADMIN_TOKEN")?,
            
            rust_env: std::env::var("RUST_ENV")
                .unwrap_or_else(|_| "development".to_string()),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}