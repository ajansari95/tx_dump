use crate::models::transaction::{ResponseData};
use std::fmt;

pub const COSMOS_API_ENDPOINT: &str = "https://lcd.cosmoshub-4.quicksilver.zone:443";


// Enum for custom error types
#[derive(Debug)]
pub enum FetchError {
    NetworkError,
    ParseError,
}

// Implementing the Display trait for FetchError
impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::NetworkError => write!(f, "Network error occurred during fetch"),
            FetchError::ParseError => write!(f, "Failed to parse the fetched data"),
        }
    }
}

/// Fetches transaction data for a given block height from the Cosmos API.
///
/// # Arguments
///
/// * `height` - The block height for which we want to retrieve transaction data.
///
/// # Returns
///
/// * `Result<ResponseData,FetchError>` - On success, returns the fetched transaction data.
///   On failure, returns a custom error indicating the type of the issue.
pub fn fetch_transactions_for_height(height: u64) -> Result<ResponseData,FetchError>{
    let url = format!("{}/cosmos/tx/v1beta1/txs?events=tx.height={}", COSMOS_API_ENDPOINT, height);

    println!("{}", url);

    let res = reqwest::blocking::get(&url).map_err(|_| FetchError::NetworkError)?;
    let res_text = res.text().map_err(|_| FetchError::ParseError)?;

    let data: Result<ResponseData, _> = serde_json::from_str(&res_text);
    data.map_err(|_| FetchError::ParseError)
}

