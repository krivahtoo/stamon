use redb::{Database, Error, ReadableTable, TableDefinition};
use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

const CONFIG_TABLE: TableDefinition<&str, &str> = TableDefinition::new("settings");

#[derive(Clone)]
pub struct ConfigStore {
    db: Arc<Database>,
}

impl ConfigStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let db = Arc::new(Database::create(path)?);
        Ok(Self { db })
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), Error> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CONFIG_TABLE)?;
            table.insert(key, value)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, Error> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CONFIG_TABLE)?;
        Ok(table.get(key)?.map(|value| value.value().to_string()))
    }

    pub fn list_settings(&self) -> Result<Vec<String>, Error> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CONFIG_TABLE)?;
        Ok(table
            .iter()?
            .map(|result| result.map(|(key, _)| key.value().to_string()))
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn delete_setting(&self, key: &str) -> Result<(), Error> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CONFIG_TABLE)?;
            table.remove(key)?;
        }
        write_txn.commit()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct EnvConfig {
    pub db_file: String,
    pub config_file: String,
    pub port: u16,

    pub jwt_secret: String,
}

impl EnvConfig {
    pub fn new() -> Result<EnvConfig, std::env::VarError> {
        let db_file = std::env::var("DB_FILE")?;
        let config_file = std::env::var("CONFIG_FILE")?;
        let jwt_secret = std::env::var("JWT_SECRET")?;

        Ok(EnvConfig {
            db_file,
            config_file,
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
