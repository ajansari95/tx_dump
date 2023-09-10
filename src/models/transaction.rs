use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::pagination::Pagination;
use super::message::Message;




#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    txs: Vec<Tx>,
    tx_responses: Vec<TxResponse>,
    pub(crate) pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tx {
    body: Body,
    auth_info: Value,
    signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    messages: Vec<Message>,
    memo: String,
    timeout_height: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxResponse {
    height: String,
    txhash: String,
    codespace: String,
    code: i32,
    data: String,
    raw_log: Value,
    logs: Value,
    gas_wanted: String,
    gas_used: String,
    timestamp: String,
}

