use std::error::Error;
use std::fmt::Display;

use chrono::{DateTime, Utc};
use csv::Writer;
use serde::Serialize;

use crate::api::dumper::{display_pretty, dump_to_csv};
use crate::api::fetcher::{fetch_by_tx_hash, fetch_transactions_for_height, fetch_transactions_for_height_range, FetchError, get_comprehensive_tx_data_for_height, get_comprehensive_tx_data_for_height_range, get_individual_txs_from_comprehensive_txs};
use crate::cmd::args;
use crate::cmd::args::{BundledMsgsRangeOpts, QueryTxAtHeightOpts, QueryTxForRangeHeightOpts};
use crate::config::config::Config;
use crate::models::transaction::{ComprehensiveTx, IndividualMsgTx, ResponseData, Translate};

/// handle_query_tx_hash Handles the query_tx_hash subcommand
pub async fn handle_query_tx_hash(config: &Config, opts: args::QueryTxHashOpts) {
    println!("Querying transaction with hash: {}", opts.hash);
    let config_clone = config.clone();
    let join_handle = tokio::task::spawn_blocking(move || fetch_by_tx_hash(&config_clone, &opts.hash));
    let result = join_handle.await;

    let data = match result {
        Ok(Ok(d)) => d,
        Ok(Err(error)) => {
            eprintln!("Failed to fetch data: {}", error);
            return;
        }
        Err(join_error) => {
            eprintln!("Task panicked: {}", join_error);
            return;
        }
    };

    let comprehensive_txs = match data.translate() {
        Ok(txs) => txs,
        Err(e) => {
            eprintln!("Error translating response data: {}", e);
            return;
        }
    };

    println!("{:?}", comprehensive_txs);
}

/// handle_query_tx_at_height Handles the query_tx_at_height subcommand
pub async fn handle_query_tx_at_height(config: &Config, opts: QueryTxAtHeightOpts) {
    println!("Querying transaction at height", );
    match opts.cmd {
        args::QueryTxAtHeightSubCommand::TxDetails(tx_details_opts) => {
            handle_tx_details_at_height(config, tx_details_opts).await.expect("fATAL: UNABLE TO HANDLE TX DETAILS AT HEIGHT");
        }
        args::QueryTxAtHeightSubCommand::MsgDetails(msg_details_opts) => {
            handle_msg_details_at_height(config, msg_details_opts).await.expect("fATAL: UNABLE TO HANDLE MSG DETAILS AT HEIGHT");
        }
    }
}

/// TransactionResult is an enum that represents the result of a transaction query
enum TransactionResult {
    SimpleData(Vec<ResponseData>),
    ComprehensiveData(Vec<ComprehensiveTx>),
}

/// handle_tx_details_at_height Handles the tx_details subcommand
 async fn handle_tx_details_at_height(config: &Config, opts: args::BundledMsgsOpts) -> Result<(), Box<dyn std::error::Error>> {
    // Start by defining the return type for the spawn_blocking
    type BlockingResult = Result<TransactionResult, FetchError>;

    // Then perform the blocking operation
    let result: BlockingResult = match (opts.common_flags.simplified, opts.common_flags.raw) {
        (Some(true), _) => {
            let config_clone = config.clone(); // Make sure Config implements Clone, or use Arc if necessary
            let height = opts.height;
            tokio::task::spawn_blocking(move || fetch_transactions_for_height(&config_clone, height)
                .map(TransactionResult::SimpleData)).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
        }
        (_, Some(true)) => {
            let config_clone = config.clone(); // Make sure Config implements Clone, or use Arc if necessary
            let height = opts.height;
            tokio::task::spawn_blocking(move || get_comprehensive_tx_data_for_height(&config_clone, height)
                .map(TransactionResult::ComprehensiveData)).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
        }
        _ => {
            eprintln!("Invalid option combination. Please check the provided flags.");
            return Err("Invalid option combination.".into());
        }
    };

    match result {
        Ok(TransactionResult::SimpleData(data)) => {
            handle_data_dump_and_display(&data, opts.dump_csv, format!("tx_dump_at_{}.csv", opts.height));
            Ok(())
        }
        Ok(TransactionResult::ComprehensiveData(data)) => {
            handle_data_dump_and_display(&data, opts.dump_csv, format!("tx_dump_at_{}.csv", opts.height));
            Ok(())
        }
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            Err(Box::new(error))
        }
    }
}


/// handle_data_dump_and_display Handles the data dump and display for the given data
fn handle_data_dump_and_display<T: Display + Serialize>(
    data: &Vec<T>,
    dump_csv_option: Option<bool>,
    filename: String,
) {
    if let Some(true) = dump_csv_option {
        if let Err(e) = dump_to_csv(data, filename) {
            eprintln!("Error while dumping to CSV: {}", e);
            return;
        }
    }
    display_pretty(data);
}

/// handle_msg_details_at_height Handles the msg_details subcommand
 async fn handle_msg_details_at_height(config: &Config, opts: args::IndividualMsgOpts) -> Result<(), Box<dyn std::error::Error>> {
    type BlockingResult = Result<Vec<ComprehensiveTx>, FetchError>;

    // Perform the blocking operation
    let comptxs: BlockingResult = {
        let config_clone = config.clone(); // Make sure Config implements Clone, or use Arc if necessary
        let height = opts.height;
        tokio::task::spawn_blocking(move || get_comprehensive_tx_data_for_height(&config_clone, height)).await?
    };

    let comptxs = match comptxs {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return Err(Box::new(error));
        }
    };
    let data = match get_individual_txs_from_comprehensive_txs(&comptxs) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return Err(Box::new(error));
        }
    };

    let mut filtered_data = match opts.filter_by_msgtype {
        Some(msg_type) => IndividualMsgTx::filter_by_type(&data, msg_type),
        None => data,
    };

    let sorted_data_by_timestamp = match opts.sort_by_timestamp {
        Some(_) => {
            IndividualMsgTx::sort_by::<DateTime<Utc>>(&mut filtered_data, false);
            filtered_data
        }
        None => filtered_data,
    };

    if opts.dump_csv.unwrap_or(false) {
        if let Err(e) = dump_to_csv_try(&sorted_data_by_timestamp, format!("msg_dump_at_{}.csv", opts.height)) {
            eprintln!("Error while dumping to CSV: {}", e);
            return Err(e.into());
        }
    }

    display_pretty(&sorted_data_by_timestamp);
    Ok(())
}

///dEBUGfUNCTION
 fn dump_to_csv_try(data: &Vec<IndividualMsgTx>, filename: String) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;
    for item in data {
        writer.serialize(item)?;
    }

    writer.flush()?;

    Ok(())
}

/// handle_query_tx_for_range_height Handles the query_tx_for_range_height subcommand
pub async fn handle_query_tx_for_range_height(config: Config, opts: QueryTxForRangeHeightOpts) {
    println!("Querying transaction at height", );
    match opts.cmd {
        args::QueryTxForRangeHeightSubCommand::TxDetails(tx_details_opts) => {
            handle_tx_details_for_range(config, tx_details_opts).await.expect("Panic: Unable to handle tx details for range");
        }
        args::QueryTxForRangeHeightSubCommand::MsgDetails(msg_details_opts) => {
            handle_msg_details_for_range(config, msg_details_opts).await.expect("Panic: Unable to handle msg details for range");
        }
    }
}


/// handle_tx_details_for_range Handles the tx_details subcommand
async fn handle_tx_details_for_range(config: Config, opts: BundledMsgsRangeOpts) -> Result<(), Box<dyn Error>> {
    let arc_config = std::sync::Arc::new(config.clone());

    let result: Result<TransactionResult, Box<dyn Error>> = match (opts.common_flags.simplified, opts.common_flags.raw) {
        (Some(true), _) => {
            let res = fetch_transactions_for_height_range(arc_config, opts.from_height, opts.to_height).await?;
            Ok(TransactionResult::SimpleData(res))
        }
        (_, Some(true)) => {
            let res = tokio::task::spawn_blocking(move || {
                get_comprehensive_tx_data_for_height_range(&config, opts.from_height, opts.to_height)
            }).await??; // Double `?`: One for JoinError and one for the result of the function
            Ok(TransactionResult::ComprehensiveData(res))
        }
        _ => {
            eprintln!("Invalid option combination. Please check the provided flags.");
            return Err("Invalid option combination.".into());
        }
    };

    match result {
        Ok(TransactionResult::SimpleData(data)) => {
            handle_data_dump_and_display(&data, opts.dump_csv, format!("tx_dump_at_{}_to_{}.csv", opts.from_height, opts.to_height));
        }
        Ok(TransactionResult::ComprehensiveData(data)) => {
            handle_data_dump_and_display(&data, opts.dump_csv, format!("tx_dump_at_{}_to_{}.csv", opts.from_height, opts.to_height));
        }
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return Err(error.into());
        }
    }

    Ok(())
}

async fn handle_msg_details_for_range(config: Config, opts: args::IndividualMsgRangeOpts) -> Result<(), Box<dyn std::error::Error>> {
    type BlockingResult = Result<Vec<ComprehensiveTx>, FetchError>;
    // Perform the blocking operation
    let comptxs: BlockingResult = {
        let config_clone = config.clone(); // Make sure Config implements Clone, or use Arc if necessary
        tokio::task::spawn_blocking(move || get_comprehensive_tx_data_for_height_range(&config_clone, opts.from_height, opts.to_height)).await?
    };

    let comptxs = match comptxs {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return Err(Box::new(error));
        }
    };

    let data = match get_individual_txs_from_comprehensive_txs(&comptxs) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return Err(Box::new(error));
        }
    };

    let mut filtered_data = match opts.filter_by_msgtype {
        Some(msg_type) => IndividualMsgTx::filter_by_type(&data, msg_type),
        None => data,
    };


    let sorted_data_by_timestamp = match opts.sort_by_timestamp {
        Some(_) => {
            IndividualMsgTx::sort_by::<DateTime<Utc>>(&mut filtered_data, false);
            filtered_data
        }
        None => filtered_data,
    };

    if opts.dump_csv.unwrap_or(false) {
        if let Err(e) = dump_to_csv_try(&sorted_data_by_timestamp, format!("msg_dump_from_{}_to_{}.csv", opts.from_height, opts.to_height)) {
            eprintln!("Error while dumping to CSV: {}", e);
            return Err(e.into());
        }
    }

    display_pretty(&sorted_data_by_timestamp);
    Ok(())
}

