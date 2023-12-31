// Import necessary libraries and modules.
use clap::{Parser};
use crate::models;
use crate::models::message::MessageType;

// Enum to represent the different message types supported.
// NOTE: Actual enum values seem to be missing in the provided snippet.

// Enum to define the order of sorting (Ascending or Descending).
#[derive(Parser, Debug, PartialEq, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

// Define common flags for querying transactions.
#[derive(Parser, Debug, Clone)]
pub struct CommonQueryFlags {
    /// Process and display data in a simplified human-readable format.
    #[clap(short, long)]
    pub simplified: Option<bool>,

    /// Raw dump of data directly from the blockchain.
    #[clap(short, long)]
    pub raw: Option<bool>,
}

// Define the primary command line arguments for the application.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Your Name")]
pub struct Opts {
    /// Custom configuration file path.
    #[clap(short, long, global=true)]
    pub config: Option<String>,

    /// Determines which subcommand to execute.
    #[clap(subcommand)]
    pub cmd: TxDumpCommand,
}

// Enum representing the main subcommands available for transactions.
#[derive(Parser, Debug)]
pub enum TxDumpCommand {
    QueryTxAtHeight(QueryTxAtHeightOpts),
    QueryTxHash(QueryTxHashOpts),
    QueryTxForRangeHeight(QueryTxForRangeHeightOpts),
}

// Struct for options related to querying a transaction by its height.
#[derive(Parser, Debug)]
pub struct QueryTxAtHeightOpts {
    /// Subcommand to query transaction details at a specific height.
    #[clap(subcommand)]
    pub cmd: QueryTxAtHeightSubCommand,
}

// Enum for subcommands under "QueryTxAtHeight".
#[derive(Parser, Debug)]
pub enum QueryTxAtHeightSubCommand {
    TxDetails(BundledMsgsOpts),
    MsgDetails(IndividualMsgOpts),
}

// Options specific to querying bundled message details by transaction height.
#[derive(Parser, Debug)]
pub struct BundledMsgsOpts {
    #[clap(flatten)]
    pub common_flags: CommonQueryFlags,
    /// Option to dump data in CSV format.
    #[clap(short, long)]
    pub dump_csv: Option<bool>,
    /// Height of the transaction to query.
    pub height: u64,
}

// Options specific to querying individual message details by transaction height.
#[derive(Parser, Debug)]
pub struct IndividualMsgOpts {
    #[clap(flatten)]
    pub common_flags: CommonQueryFlags,
    /// Option to dump data in CSV format.
    #[clap(short, long)]
    pub dump_csv: Option<bool>,
    /// Sort results by the timestamp of the transaction.
    #[clap(long)]
    pub sort_by_timestamp: Option<String>,
    /// Sort results by the gas used in the transaction.
    #[clap(long)]
    pub sort_by_gas_used: Option<String>,
    /// Filter transactions based on a specific message type.
    #[clap(long)]
    pub filter_by_msgtype: Option<models::message::MessageType>,
    /// Height of the transaction to query.
    pub height: u64,
}

// Options for querying a transaction by its unique hash.
#[derive(Parser, Debug)]
pub struct QueryTxHashOpts {
    #[clap(flatten)]
    pub common_flags: CommonQueryFlags,
    /// The hash of the transaction to query.
    pub hash: String,
}

// Options for querying transactions over a specific range of heights.
#[derive(Parser, Debug)]
pub struct QueryTxForRangeHeightOpts {
    /// Subcommand to query transaction details over a height range.
    #[clap(subcommand)]
    pub cmd: QueryTxForRangeHeightSubCommand,
}

// Enum for subcommands under "QueryTxForRangeHeight".
#[derive(Parser, Debug)]
pub enum QueryTxForRangeHeightSubCommand {
    TxDetails(BundledMsgsRangeOpts),
    MsgDetails(IndividualMsgRangeOpts),
}

// Options specific to querying bundled message details over a range of heights.
#[derive(Parser, Debug)]
pub struct BundledMsgsRangeOpts {
    #[clap(flatten)]
    pub common_flags: CommonQueryFlags,
    /// Option to dump data in CSV format.
    #[clap(short, long)]
    pub dump_csv: Option<bool>,
    /// Starting height of the transaction range to query.
    pub from_height: u64,
    /// Ending height of the transaction range to query.
    pub to_height: u64,
}

// Options specific to querying individual message details over a range of heights.
#[derive(Parser, Debug)]
pub struct IndividualMsgRangeOpts {
    #[clap(flatten)]
    pub common_flags: CommonQueryFlags,
    /// Option to dump data in CSV format.
    #[clap(short, long)]
    pub dump_csv: Option<bool>,
    /// Sort results by the timestamp of the transactions.
    #[clap(long)]
    pub sort_by_timestamp: Option<String>,
    /// Sort results by the gas used in the transactions.
    #[clap(long)]
    pub sort_by_gas_used: Option<String>,
    /// Filter transactions based on a specific message type.
    #[clap(long)]
    pub filter_by_msgtype: Option<MessageType>,
    /// Starting height of the transaction range to query.
    pub from_height: u64,
    /// Ending height of the transaction range to query.
    pub to_height: u64,
}