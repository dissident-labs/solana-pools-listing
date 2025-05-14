use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};
use tracing::{debug, error, info};

const MIGRATION_PATH: &str = "../../migrations";

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DBSettings {
    pub database_url: String,
    pub max_connections: u32,
}

pub struct Database {
    pool: Pool<Postgres>,
}

impl Default for Database {
    fn default() -> Self {
        unimplemented!(
            "Database::default() should not be called directly. Use Database::init() instead"
        )
    }
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init(database: &DBSettings) -> Result<Self>
    where
        Self: Sized;
    fn get_pool(&self) -> &Pool<Postgres>;
    async fn migrate(&self) -> Result<()>;
}
#[async_trait]
impl DatabaseTrait for Database {
    async fn init(database: &DBSettings) -> Result<Self> {
        let database_url = database.database_url.clone();
        let pool = match PgPoolOptions::new()
            .max_connections(database.max_connections)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                info!("✅ Connection to the database is successful!");
                pool
            }
            Err(err) => {
                error!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };
        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
    async fn migrate(&self) -> Result<()> {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        debug!("crate_dir: {}", crate_dir);
        let migrations = std::path::Path::new(&crate_dir).join(MIGRATION_PATH);
        let migration_results = Migrator::new(migrations)
            .await
            .unwrap()
            .run(&self.pool)
            .await;
        match migration_results {
            Ok(_) => info!("Migration success"),
            Err(error) => {
                error!("error: {}", error);
            }
        }

        Ok(())
    }
}
