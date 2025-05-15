pub mod connectors;
pub mod database;
mod entities;
mod repositories;
pub mod server;
pub mod utils;

use crate::database::{DBSettings, DatabaseTrait};
use crate::entities::financial_product::FinancialProduct;

use crate::connectors::meteora_connector::fetch_meteora_pools;
use crate::connectors::orderly_connector::fetch_orderly_vaults;
use crate::repositories::meteora_pool_repository::add_meteora_pool;
use crate::repositories::vault_repository::add_vault;
use crate::server::server::Server;
use crate::server::server::ServerSettings;
use crate::utils::logger::LoggerSettings;

use std::sync::Arc;

use clap::{command, Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AppSettings {
    pub app_name: String,
    pub debug: bool,
    pub logger: LoggerSettings,
    pub db: DBSettings,
    pub server_settings: ServerSettings,
}

const PATH: &str = ".";

#[derive(Parser)]
#[command(name = "solana_hackathon")]
#[command(about = "Blockchain data management CLI", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync vaults and pools data
    Sync,
    /// List all financial products sorted by APR
    List,
    /// Start the API server
    Server,
}

async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let configuration = utils::settings::get_configuration::<AppSettings>(PATH)?;
    let _guard = utils::logger::init_logger(None, &configuration.app_name, &configuration.logger);

    let database = database::Database::init(&configuration.db).await?;

    let db_pool = database.get_pool().clone();

    match args.command {
        Commands::Sync => {
            info!("Syncing data...");

            // Existing sync logic
            let orderly_pools = fetch_orderly_vaults().await?;
            for vault in orderly_pools {
                add_vault(&db_pool, &vault).await?;
            }

            let meteora_pools = fetch_meteora_pools().await?;
            for pool in meteora_pools {
                add_meteora_pool(&db_pool, &pool).await?;
            }
        }
        Commands::List => {
            let products = repositories::combined_repository::query_all_by_apr(&db_pool).await?;
            println!("{:<15} {:<20} {:<10}", "Type", "Name", "APR");
            println!("{}", "-".repeat(45));

            for product in products {
                match product {
                    FinancialProduct::MeteoraPool(p) => {
                        println!("{:<15} {:<20} {:.2}%", "Meteora Pool", p.pool_name, p.apr)
                    }
                    FinancialProduct::Vault(v) => {
                        println!(
                            "{:<15} {:<20} {:.2}%",
                            "Orderly-Vault", v.vault_id, v.apy_30d
                        )
                    }
                }
            }
        }
        Commands::Server {} => {
            let state_params = server::server::StateParams {
                database: Arc::new(database),
            };

            let app = Server::build(configuration.server_settings.clone(), &state_params).await;
            app.run().await;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(args).await
}
