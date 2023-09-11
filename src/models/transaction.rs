use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::message::MessageType;
use crate::models::pagination::Pagination;

use super::message::Message;

// Traits

/// Trait for translating one type into another.
/// Primarily used for transforming between transaction types.
pub trait Translate<T> {
    fn translate(&self) -> Result<Vec<T>, TranslationError>;
}

/// Trait for fields that can be sorted within a transaction.
pub trait SortableField<T>: Ord {
    fn get_field_value(tx: &T) -> &Self;
}

// Core Structures

/// Represents a comprehensive view of a transaction.
/// Contains detailed information including messages, block height, signatures and more.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComprehensiveTx {
    messages: Vec<Message>,
    height: u64,
    tx_hash: String,
    gas_used: u64,
    gas_wanted: String,
    timestamp: DateTime<Utc>,
    data: String,
    signatures: Vec<String>,
    memo: String,
    timeout_height: String,
}

/// Represents a specific message within a transaction.
/// Includes a subset of the attributes present in `ComprehensiveTx`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndividualMsgTx {
    message: Message,
    height: u64,
    tx_hash: String,
    timestamp: DateTime<Utc>,
    data: String,
    signatures: Vec<String>,
    memo: String,
    timeout_height: String,
}

// ComprehensiveTx Methods

impl ComprehensiveTx {
    /// Converts the comprehensive transaction into individual message transactions.
    fn to_individual_msg_txs(&self, message: &Message) -> IndividualMsgTx {
        IndividualMsgTx {
            message: message.clone(),
            height: self.height,
            tx_hash: self.tx_hash.clone(),
            timestamp: self.timestamp,
            data: self.data.clone(),
            signatures: self.signatures.clone(),
            memo: self.memo.clone(),
            timeout_height: self.timeout_height.clone(),
        }
    }

    /// Sorts a list of transactions based on a given sortable field.
    pub fn sort_by<T: SortableField<Self>>(transactions: &mut Vec<Self>, ascending: bool) {
        transactions.sort_by(|a, b| {
            let a_val = T::get_field_value(a);
            let b_val = T::get_field_value(b);
            if ascending {
                a_val.cmp(b_val)
            } else {
                b_val.cmp(a_val)
            }
        });
    }
}

// IndividualMsgTxs Methods

impl IndividualMsgTx {
    /// Filters the transactions based on the given message type.
    pub fn filter_by_type(txs: &[Self], msg_type: MessageType) -> Vec<Self> {
        txs.iter().filter(|tx| {
            match (&tx.message, &msg_type) {
                (Message::MsgSend { .. }, MessageType::MsgSend) => true,
                (Message::MsgDelegate { .. }, MessageType::MsgDelegate) => true,
                (Message::MsgTransfer { .. }, MessageType::MsgTransfer) => true,
                (_, MessageType::Other) => true,
                _ => false,
            }
        }).cloned().collect()
    }
}

// Trait Implementations

// Implementations for translating between different types of transactions.

// Implementation of the `Translate` trait for the `ComprehensiveTx` structure.
impl Translate<IndividualMsgTx> for ComprehensiveTx {
    fn translate(&self) -> Result<Vec<IndividualMsgTx>, TranslationError> {
        let results: Vec<IndividualMsgTx> = self.messages.iter()
            .map(|msg| self.to_individual_msg_txs(msg))
            .collect();

        Ok(results)
    }
}

// Implementation of the `Translate` trait for the `Vec<ComprehensiveTx>` structure.
impl Translate<IndividualMsgTx> for Vec<ComprehensiveTx> {
    fn translate(&self) -> Result<Vec<IndividualMsgTx>, TranslationError> {
        let mut results = Vec::new();
        for comp_tx in self {
            results.extend(comp_tx.translate()?);
        }
        Ok(results)
    }
}

// Implementation of the `Translate` trait for the `ResponseDataForHashQuery` structure.
impl Translate<ComprehensiveTx> for ResponseDataForHashQuery {
    fn translate(&self) -> Result<Vec<ComprehensiveTx>, TranslationError> {
        let comprehensive_tx = build_comprehensive_tx(&self.tx, &self.tx_response)?;
        Ok(vec![comprehensive_tx])
    }
}

// Implementation of the `Translate` trait for the `ResponseData` structure.
impl Translate<ComprehensiveTx> for ResponseData {
    fn translate(&self) -> Result<Vec<ComprehensiveTx>, TranslationError> {
        let comprehensive_txs: Result<Vec<_>, _> = self.txs.iter().zip(&self.tx_responses).map(|(tx, tx_response)| {
            build_comprehensive_tx(tx, tx_response)
        }).collect();
        comprehensive_txs
    }
}


// Implementations to extract specific sortable fields from transactions.

// Implementation of the `SortableField` trait for the `u64` type.
impl SortableField<ComprehensiveTx> for u64 {
    fn get_field_value(tx: &ComprehensiveTx) -> &Self {
        &tx.gas_used
    }
}

// Implementation of the `SortableField` trait for the `DateTime<Utc>` type.
impl SortableField<ComprehensiveTx> for DateTime<Utc> {
    fn get_field_value(tx: &ComprehensiveTx) -> &Self {
        &tx.timestamp
    }
}


// These structs represent different pieces of data and responses from the Cosmos API.

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDataForHashQuery {
    tx: Tx,
    tx_response: TxResponse,
}


// The main structure for a transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tx {
    body: Body,
    // The main content of the transaction.
    auth_info: Value,
    // Authentication information.
    signatures: Vec<String>,    // Signatures associated with the transaction.
}

// The body of a transaction, containing essential details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    messages: Vec<Message>,
    // The set of messages associated with this transaction.
    memo: String,
    // A memo or note associated with the transaction.
    timeout_height: String,       // The height at which the transaction times out.
}

// Structure for the response received after submitting a transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct TxResponse {
    height: String,
    // The height of the blockchain when the transaction was processed.
    txhash: String,
    // Unique hash identifier for the transaction.
    codespace: String,
    // A namespace for the transaction.
    code: i32,
    // The response code after processing.
    data: String,
    // Any data associated with the response.
    raw_log: Value,
    // A raw log of the transaction processing.
    logs: Value,
    // More detailed logs, possibly in a structured format.
    gas_wanted: String,
    // The amount of computational gas the transaction aimed to use.
    gas_used: String,
    // The actual amount of computational gas used.
    timestamp: String,          // Timestamp indicating when the transaction was processed.
}

// Main structure holding both the transactions and their corresponding responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    txs: Vec<Tx>,
    // A list of transactions.
    tx_responses: Vec<TxResponse>,
    // Corresponding list of responses for the transactions.
    pub(crate) pagination: Pagination,  // Pagination details if the data is part of a paged response.
}

// Helper function to build a comprehensive view of a transaction based on its details and response.
fn build_comprehensive_tx(tx: &Tx, tx_response: &TxResponse) -> Result<ComprehensiveTx, TranslationError> {
    let gas_used = tx_response.gas_used.parse().map_err(TranslationError::GasUsedParseError)?;
    let timestamp = DateTime::parse_from_rfc3339(&tx_response.timestamp)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(TranslationError::TimestampParseError)?;

    Ok(ComprehensiveTx {
        messages: tx.body.messages.clone(),
        height: tx_response.height.parse().unwrap_or_default(),
        tx_hash: tx_response.txhash.clone(),
        gas_used,
        gas_wanted: tx_response.gas_wanted.clone(),
        timestamp,
        data: tx_response.data.clone(),
        signatures: tx.signatures.clone(),
        memo: tx.body.memo.clone(),
        timeout_height: tx.body.timeout_height.clone(),
    })
}

// Error Handling

// Enum to represent different kinds of errors that can occur during translation.
pub enum TranslationError {
    GasUsedParseError(std::num::ParseIntError),
    TimestampParseError(chrono::format::ParseError),
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslationError::GasUsedParseError(e) => {
                write!(f, "Error parsing gas used: {}", e)
            }
            TranslationError::TimestampParseError(e) => {
                write!(f, "Error parsing timestamp: {}", e)
            }
        }
    }
}





#[cfg(test)]
mod tests {
    use crate::models::message::Amount;

    use super::*;

    // Mock data creation functions
    fn mock_tx() -> Tx {
        Tx {
            // Fill in fields with mock data...
            body: Body {
                messages: vec![Message::MsgSend {
                    from_address: "cosmos12".to_string(),
                    to_address: "cosmos13".to_string(),
                    amount: vec![Amount {
                        denom: "ustake".to_string(),
                        amount: "1000".to_string(),
                    }],
                }],
                memo: "start".to_string(),
                timeout_height: "ss".to_string(),
            },
            auth_info: Value::Null,
            signatures: vec![],
        }
    }

    fn mock_tx_response() -> TxResponse {
        TxResponse {
            // Fill in fields with mock data...
            height: "123".to_string(),
            txhash: "TEST#1".to_string(),
            codespace: "...".to_string(),
            code: 0,
            data: "...".to_string(),
            raw_log: Value::Null,
            logs: Value::Null,
            gas_wanted: "...".to_string(),
            gas_used: "1234".to_string(),
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_build_comprehensive_tx_valid() {
        let tx = mock_tx();
        let tx_response = mock_tx_response();

        let result = build_comprehensive_tx(&tx, &tx_response);

        assert!(result.is_ok());
        // Further assertions to check if the translated data matches the expected output.
    }

    #[test]
    fn test_build_comprehensive_tx_invalid_gas_used() {
        let tx = mock_tx();
        let mut tx_response = mock_tx_response();
        tx_response.gas_used = "invalid".to_string();  // set invalid gas_used

        let result = build_comprehensive_tx(&tx, &tx_response);

        assert!(matches!(result, Err(TranslationError::GasUsedParseError(_))));
    }

    #[test]
    fn test_build_comprehensive_tx_invalid_timestamp() {
        let tx = mock_tx();
        let mut tx_response = mock_tx_response();
        tx_response.timestamp = "invalid".to_string();  // set invalid timestamp

        let result = build_comprehensive_tx(&tx, &tx_response);

        assert!(matches!(result, Err(TranslationError::TimestampParseError(_))));
    }

    #[test]
    fn test_build_comprehensive_tx_other_message() {
        let mut tx = mock_tx();
        let tx_response = mock_tx_response();
        // set other message
        tx.body.messages = vec![Message::Other {}];

        let result = build_comprehensive_tx(&tx, &tx_response);

        assert!(result.is_ok());
    }

    fn mock_comprehensive_tx() -> ComprehensiveTx {
        ComprehensiveTx {
            messages: vec![ Message::MsgSend {
                from_address: "cosmos12".to_string(),
                to_address: "cosmos13".to_string(),
                amount: vec![Amount {
                    denom: "ustake".to_string(),
                    amount: "1000".to_string(),
                }],
            }],
            height: 0,
            tx_hash: "".to_string(),
            gas_used: 1000,
            gas_wanted: "".to_string(),
            timestamp: Utc.ymd(2023, 9, 14).and_hms(4, 5, 6),
            data: "".to_string(),
            signatures: vec![],
            memo: "".to_string(),
            timeout_height: "".to_string(),
        }
    }

    fn mock_message () -> Message {
        Message::MsgSend {
            from_address: "cosmos12".to_string(),
            to_address: "cosmos13".to_string(),
            amount: vec![Amount {
                denom: "ustake".to_string(),
                amount: "1000".to_string(),
            }],
        }
    }

    use chrono::TimeZone;
    use super::*;

    #[test]
    fn test_gas_used_sortable_field() {
        let tx = mock_comprehensive_tx();
        let gas_used: &u64 = SortableField::get_field_value(&tx);
        assert_eq!(*gas_used, 1000);
    }

    #[test]
    fn test_timestamp_sortable_field() {
        let tx = mock_comprehensive_tx();
        let timestamp: &DateTime<Utc> = SortableField::get_field_value(&tx);
        assert_eq!(*timestamp, Utc.ymd(2023, 9, 14).and_hms(4, 5, 6));
    }


    #[test]
    fn test_to_individual_msg_txs() {
        let comp_tx = mock_comprehensive_tx();
        let message = mock_message();

        let individual_msg_tx = comp_tx.to_individual_msg_txs(&message);

        // Check if the fields in the returned object match the source data
        assert_eq!(individual_msg_tx.message, message);
        assert_eq!(individual_msg_tx.height, comp_tx.height);
        assert_eq!(individual_msg_tx.tx_hash, comp_tx.tx_hash);
        assert_eq!(individual_msg_tx.timestamp, comp_tx.timestamp);
        assert_eq!(individual_msg_tx.data, comp_tx.data);
        assert_eq!(individual_msg_tx.signatures, comp_tx.signatures);
        assert_eq!(individual_msg_tx.memo, comp_tx.memo);
        assert_eq!(individual_msg_tx.timeout_height, comp_tx.timeout_height);
    }

}

