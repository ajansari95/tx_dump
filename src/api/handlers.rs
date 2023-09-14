use std::fmt::Display;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::api::dumper::{display_pretty, dump_to_csv};
use crate::api::fetcher::{fetch_by_tx_hash, fetch_transactions_for_height, fetch_transactions_for_height_range, FetchError, get_comprehensive_tx_data_for_height, get_comprehensive_tx_data_for_height_range, get_individual_txs_from_comprehensive_txs};
use crate::api::handlers::TransactionResult::ComprehensiveData;
use crate::args;
use crate::args::{BundledMsgsRangeOpts, IndividualMsgRangeOpts, QueryTxAtHeightOpts, QueryTxForRangeHeightOpts};
use crate::config::config::Config;
use crate::models::transaction::{ComprehensiveTx, IndividualMsgTx, ResponseData, Translate};

pub fn handle_query_tx_hash(config: &Config, opts: args::QueryTxHashOpts) {
    println!("Querying transaction with hash: {}", opts.hash);
    let data = match fetch_by_tx_hash(config, &opts.hash) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
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

pub fn handle_query_tx_at_height(config: &Config, opts: QueryTxAtHeightOpts) {
    println!("Querying transaction at height",);
    match opts.cmd {
        args::QueryTxAtHeightSubCommand::TxDetails(tx_details_opts) => {
            handle_tx_details_at_height(config, tx_details_opts);
        }
        args::QueryTxAtHeightSubCommand::MsgDetails(msg_details_opts) => {
            handle_msg_details_at_height(config, msg_details_opts);
        }
    }
}

// fn handle_tx_details_at_height(config: &Config, opts: args::BundledMsgsOpts) {
//     let result = match (opts.common_flags.simplified, opts.common_flags.raw) {
//         (Some(true), _) => fetch_transactions_for_height(config, opts.height),
//         (_, Some(true)) => get_comprehensive_tx_data_for_height(config, opts.height),
//         _ => {
//             eprintln!("Invalid option combination. Please check the provided flags.");
//             return;
//         }
//     };
//
//     match result {
//         Ok(data) => {
//             if let Some(true) = opts.dump_csv {
//                 if let Err(e) = dump_to_csv(&data, format!("tx_dump_at_{}.csv", opts.height)) {
//                     eprintln!("Error while dumping to CSV: {}", e);
//                     return;
//                 }
//             }
//             display_pretty(&data);
//         }
//         Err(error) => {
//             eprintln!("Failed to fetch data: {}", error);
//             return;
//         }
//     }
// }

enum TransactionResult {
    SimpleData(Vec<ResponseData>),
    ComprehensiveData(Vec<ComprehensiveTx>),
}

fn handle_tx_details_at_height(config: &Config, opts: args::BundledMsgsOpts) {
    let result: Result<TransactionResult, FetchError> = match (opts.common_flags.simplified, opts.common_flags.raw) {
        (Some(true), _) => fetch_transactions_for_height(config, opts.height)
            .map(TransactionResult::SimpleData),
        (_, Some(true)) => get_comprehensive_tx_data_for_height(config, opts.height)
            .map(TransactionResult::ComprehensiveData),
        _ => {
            eprintln!("Invalid option combination. Please check the provided flags.");
            return;
        }
    };

    match result {
        Ok(TransactionResult::SimpleData(data)) => {
            handle_data_dump_and_display(&data, opts.dump_csv, format!("tx_dump_at_{}.csv", opts.height));
        }
        Ok(TransactionResult::ComprehensiveData(data)) => {
            handle_data_dump_and_display(&data, opts.dump_csv, format!("tx_dump_at_{}.csv", opts.height));
        }
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
        }
    }
}

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

fn handle_msg_details_at_height(config: &Config, opts: args::IndividualMsgOpts) {
    let comptxs = match get_comprehensive_tx_data_for_height(config, opts.height) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return;
        }
    };

    let data = match get_individual_txs_from_comprehensive_txs(&comptxs) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return;
        }
    };

    let mut filtered_data = match opts.filter_by_msgtype {
        Some(msg_type) => IndividualMsgTx::filter_by_type(&data, msg_type),
        None => data,
    };

    let mut sorted_data_by_timestamp = match opts.sort_by_timestamp {
        Some(_) => {
            IndividualMsgTx::sort_by::<DateTime<Utc>>(&mut filtered_data, false);
            filtered_data
        },
        None => filtered_data,
    };


    if opts.dump_csv.unwrap_or(false) {
        if let Err(e) = dump_to_csv(&sorted_data_by_timestamp, format!("msg_dump_at_{}.csv", opts.height)) {
            eprintln!("Error while dumping to CSV: {}", e);
            return;
        }
    }
    display_pretty(&sorted_data_by_timestamp);
}

pub fn handle_query_tx_for_range_height(config: Config, opts: QueryTxForRangeHeightOpts) {
    println!("Querying transaction at height",);
    match opts.cmd {
        args::QueryTxForRangeHeightSubCommand::TxDetails(tx_details_opts) => {
            handle_tx_details_for_range(config, tx_details_opts);
        }
        args::QueryTxForRangeHeightSubCommand::MsgDetails(msg_details_opts) => {
            handle_msg_details_for_range(config,msg_details_opts );
        }
    }
}


async fn handle_tx_details_for_range(config: Config, opts: BundledMsgsRangeOpts) {
    let arc_config = std::sync::Arc::new(config.clone());
    let result = match (opts.common_flags.simplified, opts.common_flags.raw) {
        (Some(true), _) => fetch_transactions_for_height_range(arc_config, opts.from_height, opts.to_height).await
            .map(TransactionResult::SimpleData),
        (_, Some(true)) => get_comprehensive_tx_data_for_height_range(&config, opts.from_height,opts.to_height)
            .map(TransactionResult::ComprehensiveData),
        _ => {
            eprintln!("Invalid option combination. Please check the provided flags.");
            return;
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
        }
    }
}

async fn handle_msg_details_for_range(config: Config, opts: args::IndividualMsgRangeOpts) {
    let comptxs: Vec<ComprehensiveTx> = match get_comprehensive_tx_data_for_height_range(&config, opts.from_height, opts.to_height) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return;
        }
    };

    let data = match get_individual_txs_from_comprehensive_txs(&comptxs) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch data: {}", error);
            return;
        }
    };

    let mut filtered_data = match opts.filter_by_msgtype {
        Some(msg_type) => IndividualMsgTx::filter_by_type(&data, msg_type),
        None => data,
    };

    let mut sorted_data_by_timestamp = match opts.sort_by_timestamp {
        Some(_) => {
            IndividualMsgTx::sort_by::<DateTime<Utc>>(&mut filtered_data, false);
            filtered_data
        },
        None => filtered_data,
    };

    if opts.dump_csv.unwrap_or(false) {
        if let Err(e) = dump_to_csv(&sorted_data_by_timestamp, format!("msg_dump_at_{}_to_{}.csv", opts.from_height, opts.to_height)) {
            eprintln!("Error while dumping to CSV: {}", e);
            return;
        }
    }
}

