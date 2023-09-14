
> **_Note:_** The CSV export feature for composite types is currently experiencing issues and might not work as expected. still WIP.

## Can track my learning progress [here](./LEARNINGS.md)

## `tx_dump` CLI Documentation

The `tx_dump` CLI is a versatile tool to query blockchain data with various filters.

## Installation

To install `tx_dump`, you'll need to compile it from source:

### Pre-requisites:

- Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Steps:

1. Clone the repository:
   ```bash
   git clone www.github.com/ajansari95/tx_dump
   cd tx_dump
   ```

2. Compile the binary:
   ```bash
   cargo build --release
   ```

3. Move the compiled binary to your desired location (optional):
   ```bash
   cp target/release/tx_dump /usr/local/bin/
   ```

4. Verify the installation:
   ```bash
   tx_dump --help
   ```

## Usage:

```bash
tx_dump [OPTIONS] <COMMAND>
```

## Main Commands:

- `query-tx-at-height`: Query transactions by a specific block height.
- `query-tx-hash`: Query a specific transaction using its hash.
- `query-tx-for-range-height`: Query transactions within a specified range of block heights.
- `help`: Print the help message or the help of the given subcommand(s).

### `query-tx-hash`

Retrieve details for a specific transaction hash.

```bash
tx_dump query-tx-hash [FLAGS] [OPTIONS] <hash>
```

#### Arguments:

- `hash`: The hash string of the transaction you wish to query.

### `query-tx-at-height`

Retrieve transaction details at a specified block height. This command has two variants:

- `tx-details`: Get details for individual transactions.
- `msg-details`: Get all messages for the transactions.

#### Usage:

```bash
tx_dump query-tx-at-height [VARIANT] [FLAGS] [OPTIONS] <height>
```

### `query-tx-for-range-height`

Retrieve transaction details within a specified range of block heights.

#### Usage:

```bash
tx_dump query-tx-for-range-height [VARIANT] [FLAGS] [OPTIONS] --from-height <start_height> --to-height <end_height>
```

## Enhanced `msg-details` Command Documentation

#### Usage for a specific height:

```bash
tx_dump query-tx-at-height msg-details [FLAGS] [OPTIONS] <height>
```

#### Usage for a height range:

```bash
tx_dump query-tx-for-range-height msg-details [FLAGS] [OPTIONS] --from-height <start_height> --to-height <end_height>
```

#### Flags:

- `--simplified`: Display data in a human-readable format.
- `--raw`: Provide a raw dump of data.
- `--dump-csv`: Dump data in CSV format.

#### Options:

- `--config`: Custom configuration file path.
- `--filter-by-msgtype`: Filter by message type.
- `--sort-by-timestamp`: Sort by transaction timestamp.
- `--sort-by-gas-used`: Sort by gas used.

#### Example:

```bash
tx_dump msg-details --config ./config.toml --raw true 16990463 --filter-by-msgtype="MsgSend"
```

---

