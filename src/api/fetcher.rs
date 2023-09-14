use std::fmt;
use std::fmt::{Display};
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task;

use crate::config::config::Config;
use crate::models::transaction::{ComprehensiveTx, ResponseData, ResponseDataForHashQuery, Translate,IndividualMsgTx};



// Enum for custom error types
#[derive(Debug)]
pub enum FetchError {
    NetworkError,
    ParseError,
    TranslateError,
    TaskFailure(String)
}

impl FetchError {
    #![allow(dead_code)]
    fn new(msg: &str) -> Self {
        FetchError::TaskFailure(msg.to_string())
    }
}
impl std::error::Error for FetchError {}


// Implementing the Display trait for FetchError
impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::NetworkError => write!(f, "Network error occurred during fetch"),
            FetchError::ParseError => write!(f, "Failed to parse the fetched data"),
            FetchError::TranslateError => write!(f, "Failed to translate the fetched data"),
            FetchError::TaskFailure(msg) => write!(f, "Task failure: {}", msg),
        }
    }
}

/// Fetches transaction data for a specific block height from the Cosmos SDK REST endpoint.
///
/// This function communicates with the Cosmos SDK REST API to obtain transaction details
/// corresponding to a specific block height (`height`). It constructs the necessary URL
/// and initiates a paginated fetch to retrieve all transaction details for the given height.
///
/// If any errors arise during the network request or JSON parsing process, the function
/// returns a `FetchError`.
///
/// # Arguments
///
/// * `config` - A reference to `Config` struct containing the URL of the Cosmos API.
///
/// * `height` - A `u64` value representing the block height of interest.
///
/// # Returns
///
/// A `Result` containing a `Vec` of `ResponseData` on success. If there are any issues,
/// it returns an `Err` variant with the `FetchError`.
///
/// # Examples
///
/// The following is a mock demonstration and doesn't interact with the actual Cosmos API:
///
/// ```rust
/// # use your_crate_name::{fetch_transactions_for_height, ResponseData, FetchError};
/// # fn mock_fetch_transactions_for_height(config: &Config ,height: u64) -> Result<Vec<ResponseData>, FetchError> {
/// #     let mock_data = vec![ResponseData { /* ... fill with mock data ... */ }];
/// #     Ok(mock_data)
/// # }
/// #
/// let block_height = 1234;
/// match mock_fetch_transactions_for_height(config,block_height) {
///     Ok(transactions) => {
///         // Handle the retrieved transactions
///         for transaction in transactions {
///             println!("{:?}", transaction);
///         }
///     },
///     Err(e) => {
///         // Handle the error
///         println!("Failed to fetch: {:?}", e);
///     }
/// }
/// ```
///
/// # Errors
///
/// This function can return `FetchError::NetworkError` if there's a problem with the network
/// request, and `FetchError::ParseError` if there's an issue parsing the JSON response.
///
/// # Panics
///
/// This function does not intentionally panic. However, unexpected changes in the Cosmos SDK
/// API format or misuse of underlying libraries could lead to unanticipated panics.
///
/// # Safety
///
/// This function doesn't perform unsafe operations. Ensure that the returned `Result`
/// is managed properly in the calling context to address potential errors.
pub fn fetch_transactions_for_height(config: &Config, height: u64) -> Result<Vec<ResponseData>, FetchError> {
    let mut all_data = Vec::new();
    let next_key: Option<String> = None;

    loop {
        let url = if let Some(ref nk) = next_key {
            format!("{}/cosmos/tx/v1beta1/txs?events=tx.height={}&pagination_key={}", config.url(), height, nk)
        } else {
            format!("{}/cosmos/tx/v1beta1/txs?events=tx.height={}", config.url(), height)
        };

        println!("{}", url);

        let res = reqwest::blocking::get(&url).map_err(|_| FetchError::NetworkError)?;
        let res_text = res.text().map_err(|_| FetchError::ParseError)?;

        let data: ResponseData = serde_json::from_str(&res_text).map_err(|_| FetchError::ParseError)?;
        // Check if there's a next page
        let next_key = data.pagination.next_key.clone();

        // Now, push the data to all_data
        all_data.push(data);

        // Check if there's a next page
        if next_key.is_none() {
            break;
        }
    }

    Ok(all_data)
}

/// Fetches transaction data for a given block height from the Cosmos API.
///
/// # Arguments
///
/// * `config` - A reference to `Config` struct containing the URL of the Cosmos API.
/// * `height` - The block height for which we want to retrieve transaction data.
///
/// # Returns
///
/// * `Result<ResponseData, FetchError>` - On success, returns the fetched transaction data.
///   On failure, returns a custom error indicating the type of the issue.
///
/// # Examples
///
/// Note: This is a demonstration and doesn't interact with the actual Cosmos API.
///
/// ```
/// # use your_crate_name::{fetch_transactions_for_height, ResponseData, FetchError};
/// # use std::collections::HashMap;
/// #
/// # fn mock_fetch_transactions_for_height(config: &Config ,height: u64) -> Result<Vec<ResponseData>, FetchError> {
/// #     let mock_data = vec![ResponseData { /* ... fill with mock data ... */ }];
/// #     Ok(mock_data)
/// # }
/// #
/// let block_height = 1234;
/// match mock_fetch_transactions_for_height(config: &config,block_height) {
///     Ok(data) => {
///         // Handle the retrieved data
///         for transaction in data {
///             println!("{:?}", transaction);
///         }
///     },
///     Err(e) => {
///         // Handle the error
///         println!("Failed to fetch: {:?}", e);
///     }
/// }
/// ```
pub async fn fetch_transactions_for_height_range(config: Arc<Config>, start_height: u64, end_height: u64) -> Result<Vec<ResponseData>, FetchError> {
    let mut all_data = Vec::new();

    // Create a semaphore with 10 permits
    let semaphore = Arc::new(Semaphore::new(10));

    let handles: Vec<_> = (start_height..=end_height).map(|height| {
        let config = config.clone(); // This clones the Arc, not the Config itself
        let sem_clone = semaphore.clone();
        task::spawn_blocking(move || {
            // Acquire a permit
            let _permit = sem_clone.acquire();
            // Once we have a permit, fetch the data
            fetch_transactions_for_height(&config, height)
        })
    }).collect();

    for handle in handles {
        match handle.await {
            Ok(data) => all_data.extend(data?),
            Err(_) => return Err(FetchError::TaskFailure(String::from("Task failed"))),
        };
    }

    Ok(all_data)
}
/// Fetches transaction data from the Cosmos SDK REST endpoint based on a given transaction hash.
///
/// This function interfaces with the Cosmos SDK REST API to retrieve transaction details
/// for a specific transaction hash (`tx_hash`). It builds a URL using the provided hash,
/// sends an HTTP GET request, and then attempts to parse the received JSON response
/// into a `ResponseDataForHashQuery` structure.
///
/// Any encountered network issues, such as connection failures or timeouts, will result
/// in a `FetchError::NetworkError`. Similarly, any problems during the JSON parsing phase
/// will yield a `FetchError::ParseError`.
///
/// # Arguments
///
/// * `config` - A reference to `Config` struct containing the URL of the Cosmos API.
/// * `tx_hash` - A string slice representing the transaction hash of interest.
///
/// # Returns
///
/// A `Result` that's `Ok` if the transaction data was fetched successfully, containing
/// the `ResponseDataForHashQuery`. If there were any problems, it returns an `Err`
/// variant, holding the `FetchError`.
///
/// # Examples
///
/// The following is a mock demonstration and doesn't interact with the actual Cosmos API:
///
/// ```rust
/// # use your_crate_name::{fetch_by_tx_hash, ResponseDataForHashQuery, FetchError};
/// # fn mock_fetch_by_tx_hash(config: &Config,tx_hash: &str) -> Result<ResponseDataForHashQuery, FetchError> {
/// #     let mock_data = ResponseDataForHashQuery { /* ... fill with mock data ... */ };
/// #     Ok(mock_data)
/// # }
/// #
/// let tx_hash = "your_mock_transaction_hash_here";
/// match mock_fetch_by_tx_hash(config,tx_hash) {
///     Ok(data) => {
///         // Handle the retrieved data
///         println!("{:?}", data);
///     },
///     Err(e) => {
///         // Handle the error
///         println!("Failed to fetch: {:?}", e);
///     }
/// }
/// ```
///
/// # Errors
///
/// This function can return `FetchError::NetworkError` if there's a problem with the network
/// request, and `FetchError::ParseError` if there's an issue parsing the JSON response.
///
/// # Panics
///
/// This function does not intentionally panic, but misuse of underlying libraries
/// or unexpected changes in the Cosmos SDK API format might cause unforeseen panics.
///
/// # Safety
///
/// This function doesn't perform unsafe operations. Ensure you handle the returned
/// `Result` appropriately in the calling context to manage any potential errors.
pub fn fetch_by_tx_hash(config: &Config, tx_hash: &str) -> Result<ResponseDataForHashQuery, FetchError> {
    let url = format!("{}/cosmos/tx/v1beta1/txs/{}", config.url(), tx_hash);

    // Try making the HTTP request
    let res = reqwest::blocking::get(&url).map_err(|e| {
        eprintln!("Network error: {}", e);  // Log the actual error for more context
        FetchError::NetworkError
    })?;

    // Ensure we received a successful status code before trying to parse
    if !res.status().is_success() {
        return Err(FetchError::NetworkError);
    }

    // Attempt to get the response text
    let res_text = res.text().map_err(|e| {
        eprintln!("Error retrieving text from response: {}", e);
        FetchError::ParseError
    })?;

    // Try parsing the JSON response
    let data: ResponseDataForHashQuery = serde_json::from_str(&res_text).map_err(|e| {
        eprintln!("JSON parsing error: {}", e);
        FetchError::ParseError
    })?;

    Ok(data)
}

/// Fetches and translates a transaction's data into its comprehensive format using its hash.
///
/// Given a `Config` instance and a transaction hash, this function first fetches the transaction data
/// using `fetch_by_tx_hash` method. Post fetching, it translates the raw data into a `ComprehensiveTx` format.
///
/// # Parameters
///
/// * `config`: A reference to the configuration used to access necessary endpoints or services.
/// * `tx_hash`: A string slice that represents the transaction hash.
///
/// # Returns
///
/// * `Ok(Vec<ComprehensiveTx>)`: A successful result containing a vector of `ComprehensiveTx` objects.
/// * `Err(FetchError)`: An error result indicating a problem encountered during the fetch or translation process.
///                      The error might be due to network issues, parsing problems, or translation failures.
///
/// # Errors
///
/// This function can return `FetchError::TranslateError` if there's a problem translating the raw data into `ComprehensiveTx` format.
///
/// # Example
///
/// ```rust
/// # use your_crate_name::Config;
/// let config = Config::new();
/// let tx_hash = "some_hash_string";
/// let result = get_comprehensive_tx_data_for_hash(&config, tx_hash);
/// match result {
///     Ok(data) => println!("Transaction data: {:?}", data),
///     Err(e) => eprintln!("Failed to fetch transaction data: {}", e),
/// }
/// #![allow(dead_code)]
pub fn get_comprehensive_tx_data_for_hash(config: &Config, tx_hash: &str) -> Result<Vec<ComprehensiveTx>, FetchError> {
    // translate to comprehensive and handle the error
    let data = fetch_by_tx_hash(config, tx_hash)?;
    data.translate().map_err(|_e| {
        FetchError::TranslateError
    })
}

/// Fetches and translates a transaction's data into its comprehensive format using its height.
pub fn get_comprehensive_tx_data_for_height(config: &Config, height: u64) -> Result<Vec<ComprehensiveTx>, FetchError> {
    let data = fetch_transactions_for_height(config, height)?;
    let mut comprehensive_txs = Vec::new();
    for response_data in data {
        let mut txs = response_data.translate().map_err(|_e| {
            FetchError::TranslateError
        })?;
        comprehensive_txs.append(&mut txs);
    }
    Ok(comprehensive_txs)
}

pub fn get_comprehensive_tx_data_for_height_range(config: &Config, start_height: u64, end_height: u64) -> Result<Vec<ComprehensiveTx>, FetchError> {
    let mut comprehensive_txs = Vec::new();
    for height in start_height..=end_height {
        let data = fetch_transactions_for_height(config, height)?;
        for response_data in data {
            let mut txs = response_data.translate().map_err(|_e| {
                FetchError::TranslateError
            })?;
            comprehensive_txs.append(&mut txs);
        }
    }
    Ok(comprehensive_txs)
}

pub fn get_individual_txs_from_comprehensive_txs(comprehensive_txs: &Vec<ComprehensiveTx>) -> Result<Vec<IndividualMsgTx>, FetchError> {
    let mut individual_msg_txs = Vec::new();
    individual_msg_txs = comprehensive_txs.translate().map_err(|_e| {
        FetchError::TranslateError
    })?;
    Ok(individual_msg_txs)
}






