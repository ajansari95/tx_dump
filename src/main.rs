use std::io;

use serde_json;

use api::fetcher;
use api::fetcher::FetchError;

mod api;
mod models;

fn main() {

    // Get height-input from user
    let height = match get_user_input() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error getting user input: {}", e);
            return;
        }
    };

    // Fetch transactions for height
    match fetcher::fetch_transactions_for_height(height) {
        Ok(res_text) => {
            // Pretty print using serde
            let pretty = serde_json::to_string_pretty(&res_text)
                .expect("Error converting result to pretty string");
            println!("{}", pretty);
        }
        Err(FetchError::NetworkError) => {
            eprintln!("Network error occurred while fetching transactions.");
        }
        Err(FetchError::ParseError) => {
            eprintln!("Error parsing response from server.");
        }
    }
}

fn get_user_input() -> Result<u64, io::Error> {
    let mut input = String::new();
    println!("Enter height: ");
    io::stdin().read_line(&mut input)?;
    let height = input.trim().parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Please type a number!"))?;
    Ok(height)
}
