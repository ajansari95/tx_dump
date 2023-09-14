use std::str::FromStr;
use clap::Parser;
use serde::{Deserialize, Serialize};


/// Represents various message types from the CosmosSDK.
/// The `Message` enum is used to deserialize different message types based on the `@type` field
/// in the JSON data. Each variant of the enum corresponds to a specific message type.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "@type")]
pub enum Message {
    #[serde(rename = "/cosmos.bank.v1beta1.MsgSend")]
    MsgSend {
        from_address: String,
        to_address: String,
        amount: Vec<Amount>,
    },

    #[serde(rename = "/cosmos.staking.v1beta1.MsgDelegate")]
    MsgDelegate {
        delegator_address: String,
        validator_address: String,
        amount: Amount,
    },

    #[serde(rename = "/ibc.applications.transfer.v1.MsgTransfer")]
    MsgTransfer {
        source_port: String,
        source_channel: String,
        token: Amount,
        sender: String,
        receiver: String,
        timeout_height: TimeoutHeight,
        timeout_timestamp: String,
        memo: String,
    },

    /// A fallback variant for any unexpected message types.
    #[serde(other)]
    Other,

}

/// Represents the `amount` field in the `MsgSend` and `MsgDelegate` message types.
#[derive(Debug,PartialEq ,Serialize, Deserialize, Clone)]
pub struct Amount {
    pub denom: String,
    pub amount: String,
}



/// Represents various message types from the CosmosSDK.
#[derive(Debug, Serialize, Deserialize, Clone,Parser,PartialEq)]
pub enum MessageType {
    MsgSend,
    MsgDelegate,
    MsgTransfer,
    Other,
}

/// Implementation to convert string slices to MsgType
impl FromStr for MessageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MsgSend" => Ok(MessageType::MsgSend),
            "MsgDelegate" => Ok(MessageType::MsgDelegate),
            "MsgTransfer" => Ok(MessageType::MsgTransfer),
            "Other" => Ok(MessageType::Other),
            _ => Err(format!("'{}' is not a valid MsgType value", s)),
        }
    }
}

/// Represents the `timeout_height` field in the `MsgTransfer` message type.
#[derive(Debug, PartialEq,Serialize, Deserialize, Clone)]
pub struct TimeoutHeight {
    revision_number: String,
    revision_height: String,
}