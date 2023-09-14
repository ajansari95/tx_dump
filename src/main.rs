use std::path::PathBuf;

use clap::Parser;

use cmd::args::Opts;

use crate::cmd::args::TxDumpCommand;
use crate::config::config::Config;

mod api;
mod models;
mod config;
mod cmd;


#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    // Declare an optional Config variable
    let mut g_config: Option<Config> = None;

    // If the --config flag is set, attempt to read the configuration from the specified file.
    if let Some(cfg) = opts.config {
        let config = match Config::from_file(&cfg) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                eprintln!("Error: No config provides {}", e);
                None
            }
        };
        // Reading configuration from file ok!
        if let Some(cfg) = &config {
            println!("Loaded Config: {:?}", cfg);
            g_config = config;
        }

        println!("Loaded Config: {:?}", g_config);
    }
    println!("Loaded Config: {:?}", g_config);


    match opts.cmd {
        TxDumpCommand::QueryTxAtHeight(query_height_opts) => api::handlers::handle_query_tx_at_height(&g_config.unwrap(), query_height_opts).await,
        TxDumpCommand::QueryTxHash(query_hash_opts) => api::handlers::handle_query_tx_hash(&g_config.unwrap(), query_hash_opts).await,
        TxDumpCommand::QueryTxForRangeHeight(query_range_height_opts) => api::handlers::handle_query_tx_for_range_height(g_config.unwrap(), query_range_height_opts).await,
    }
}


fn get_default_config_path() -> PathBuf {
    // let args: TxDumpArgs = TxDumpArgs::parse();
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".tx_dump");
            path.push("config.toml");
            path
        }
        None => {
            eprintln!("Warning: Unable to determine home directory. Using current directory for config path.");
            PathBuf::from("./config.toml")
        }
    }
}