use std::{fs, io};
use chrono::{DateTime, Utc};

use api::fetcher;
use api::fetcher::FetchError;
use models::transaction::{Translate};
use crate::api::fetcher::fetch_by_tx_hash;
use crate::models::message::MessageType;
use crate::models::transaction::{ComprehensiveTx, IndividualMsgTx};

mod api;
mod models;
mod config;
mod args;



use std::env;
use std::env::home_dir;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use crate::config::config::Config;
use clap::{Command, Parser ,Arg,ArgMatches};
use args::Opts;
use crate::args::TxDumpCommand;

// #[tokio::main]
// async fn main() {
//     // Fetch transactions for height
//     let new_config = Config {
//         url: "https://lcd.cosmoshub-4.quicksilver.zone:443".to_string(),
//     };
//     let config = Arc::new(new_config);
//
//     match fetcher::fetch_transactions_for_height_range(config, 16990463, 16990468).await {
//         Ok(response_data_vec) => {
//             println!("Received transactions: {:?}", response_data_vec);
//         }
//         Err(FetchError::NetworkError) => {
//             eprintln!("Network error occurred while fetching transactions.");
//         }
//         Err(FetchError::ParseError) => {
//             eprintln!("Error parsing response from server.");
//         }
//         // Add other error handlers if there are more error variants
//         Err(e) => {
//             eprintln!("An error occurred: {:?}", e);
//         }
//     }
// }


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
                eprintln!("Error: {}", e);
                None
            }
        };
        // Reading configuration from file ok!
        if let Some(cfg) = &config{
            println!("Loaded Config: {:?}", cfg);
            g_config = config;
        }

        println!("Loaded Config: {:?}", g_config);

    }
    println!("Loaded Config: {:?}", g_config);


    match opts.cmd {
        TxDumpCommand::QueryTxAtHeight(query_height_opts) => api::handlers::handle_query_tx_at_height(&g_config.unwrap(),query_height_opts),
        TxDumpCommand::QueryTxHash(query_hash_opts) => api::handlers::handle_query_tx_hash(&g_config.unwrap(),query_hash_opts),
        TxDumpCommand::QueryTxForRangeHeight(query_range_height_opts) => api::handlers::handle_query_tx_for_range_height(g_config.unwrap(),query_range_height_opts),
    }
}


fn get_default_config_path() -> PathBuf {
    // let args: TxDumpArgs = TxDumpArgs::parse();
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".tx_dump");
            path.push("config.toml");
            path
        },
        None => {
            eprintln!("Warning: Unable to determine home directory. Using current directory for config path.");
            PathBuf::from("./config.toml")
        },
    }
}
// fn main() {
//     let config_content = fs::read_to_string("config.toml").unwrap_or_else(|_| toml::to_string(&Config::default()).unwrap());
//     let config: Config = toml::from_str(&config_content).expect("Failed to parse config file");
//
//     println!("Config: {:?}", config);
//     let args: Vec<String> = env::args().collect();
//
//     // Check if we have enough arguments to at least specify the function to run.
//     if args.len() < 2 {
//         eprintln!("Usage: {} [height|range|hash] <additional arguments>", args[0]);
//         return;
//     }
//
//     match args[1].as_str() {
//         "height" => {
//             if args.len() != 3 {
//                 eprintln!("Usage: {} height <block_height>", args[0]);
//                 return;
//             }
//             let height = args[2].parse::<u64>().expect("Please provide a valid block height.");
//             match fetcher::fetch_transactions_for_height(&config,height) {
//                 Ok(data) => println!("{:?}", data),
//                 Err(e) => eprintln!("Error: {:?}", e),
//             }
//         }
//         "range" => {
//             if args.len() != 4 {
//                 eprintln!("Usage: {} range <start_height> <end_height>", args[0]);
//                 return;
//             }
//             let start_height = args[2].parse::<u64>().expect("Please provide a valid start block height.");
//             let end_height = args[3].parse::<u64>().expect("Please provide a valid end block height.");
//             match fetcher::fetch_transactions_for_height_range(&config,start_height, end_height) {
//                 Ok(data) => println!("{:?}", data),
//                 Err(e) => eprintln!("Error: {:?}", e),
//             }
//         }
//         "hash" => {
//             if args.len() != 3 {
//                 eprintln!("Usage: {} hash <tx_hash>", args[0]);
//                 return;
//             }
//             let tx_hash = &args[2];
//             match fetcher::fetch_by_tx_hash(&config,tx_hash) {
//                 Ok(data) => println!("{:?}", data),
//                 Err(e) => eprintln!("Error: {:?}", e),
//             }
//         }
//         _ => {
//             eprintln!("Unknown command. Usage: {} [height|range|hash] <additional arguments>", args[0]);
//         }
//     }
// }


// fn main() {
//     let hash = "7B374047C99D1925FAD31DF1C36CFE74E49B54AA22841E2FD201F98173A41517";
//     // Get height-input from user
//
//     match fetch_by_tx_hash(hash){
//         Ok(response_data) => {
//             let comprehensive_txs = response_data.translate();
//             let  txs = match comprehensive_txs{
//                 Ok(txs) => txs,
//                 Err(e) => {
//                     eprintln!("Error translating response data: {}", e);
//                     return;
//                 }
//             };
//             let  itms =  match txs.translate() {
//                 Ok(individual_msg_txs) => {
//                     individual_msg_txs
//                 },
//                 Err(e) => {
//                     eprintln!("Error during translation: {}", e);
//                     return;
//                 }
//             };
//             let filtered_txs = IndividualMsgTxs::filter_by_type(&itms, MessageType::MsgSend);
//             for tx in &filtered_txs {
//                 println!("{:?}", tx);
//             }
//         },
//         Err(e) => {
//             eprintln!("Error fetching transaction: {}", e);
//             return;
//         }
//     }
//
//
//     let height = match get_user_input() {
//         Ok(h) => h,
//         Err(e) => {
//             eprintln!("Error getting user input: {}", e);
//             return;
//         }
//     };
//
//     // Fetch transactions for height
//     match fetcher::fetch_transactions_for_height_range(height, height+5) {
//         Ok(response_data_vec) => {
//             for response_data in response_data_vec {
//                 // Translate each ResponseData into ComprehensiveTx
//                 let  comprehensive_txs = response_data.translate();
//
//               let mut txs = match comprehensive_txs{
//                     Ok(txs) => txs,
//                     Err(e) => {
//                         eprintln!("Error translating response data: {}", e);
//                         return;
//                     }
//                 };
//
//                let  itms =  match txs.translate() {
//                     Ok(individual_msg_txs) => {
//                         individual_msg_txs
//                     },
//                     Err(e) => {
//                         eprintln!("Error during translation: {}", e);
//                         return;
//                     }
//                 };
//
//                 let filtered_txs = IndividualMsgTxs::filter_by_type(&itms, MessageType::MsgSend);
//                 for tx in &filtered_txs {
//                     println!("{:?}", tx);
//                 }
//
//
//                 ComprehensiveTx::sort_by::<DateTime<Utc>>(&mut txs, false);
//                 for tx in &txs {
//                     println!("{:?}", tx);
//                 }
//
//
//             }
//         }
//         Err(FetchError::NetworkError) => {
//             eprintln!("Network error occurred while fetching transactions.");
//         }
//         Err(FetchError::ParseError) => {
//             eprintln!("Error parsing response from server.");
//         }
//     }
// }
//
// fn get_user_input() -> Result<u64, io::Error> {
//     let mut input = String::new();
//     println!("Enter height: ");
//     io::stdin().read_line(&mut input)?;
//     let height = input.trim().parse()
//         .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Please type a number!"))?;
//     Ok(height)
// }
