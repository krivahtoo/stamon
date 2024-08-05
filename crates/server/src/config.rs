use std::{path::PathBuf, sync::OnceLock};

#[derive(Debug)]
pub struct EnvConfig {
    pub data_path: PathBuf,

    pub assets_path: PathBuf,

    pub db_file: String,
    pub port: u16,

    pub jwt_secret: String,
}

impl EnvConfig {
    pub fn new() -> Result<EnvConfig, Box<dyn std::error::Error>> {
        let data_path = PathBuf::from(std::env::var("DATA_PATH")?);

        if !data_path.exists() {
            std::fs::create_dir_all(&data_path)?;
        }

        let assets_path =
            PathBuf::from(std::env::var("ASSETS_PATH").unwrap_or("assets".to_string()));

        let db_file = format!(
            "sqlite://{}/stamon.db",
            data_path.as_os_str().to_str().unwrap()
        );
        let jwt_secret = std::env::var("JWT_SECRET")?;

        Ok(EnvConfig {
            data_path,
            assets_path,
            db_file,
            port: 3000,
            jwt_secret,
        })
    }
}

pub fn env_config() -> &'static EnvConfig {
    static CONFIG: OnceLock<EnvConfig> = OnceLock::new();
    CONFIG.get_or_init(|| {
        EnvConfig::new().unwrap_or_else(|e| panic!("Error loading env config: {e}"))
    })
}
