use std::process;

#[derive(Debug, thiserror::Error)]
pub enum GumroadError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("No access token provided. Use --access-token, GUMROAD_ACCESS_TOKEN env var, or `gumroad auth login`.")]
    NoToken,
}

impl GumroadError {
    pub fn exit_code(&self) -> i32 {
        match self {
            GumroadError::NoToken => 2,
            GumroadError::Api(_) => 3,
            GumroadError::Http(_) => 4,
            GumroadError::Config(_) => 5,
            _ => 1,
        }
    }

    pub fn exit(&self) -> ! {
        eprintln!("Error: {self}");
        process::exit(self.exit_code());
    }
}

pub type Result<T> = std::result::Result<T, GumroadError>;
