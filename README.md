# Wallet Utilities

**Wallet Utilities** is a Rust library designed for decoding ABI-encoded data based on Ethereum function selectors. It allows you to dynamically decode calldata by matching it with known function selectors stored in a JSON file.

## Features

-   **Dynamic ABI Decoding**: Decodes calldata based on function selectors provided in a JSON file.
-   **Supports Multiple Formats**: Handles both simple and batch types as defined in the JSON file. ( Supports only simpleWallet atm )
-   **Easy Integration**: Can be seamlessly integrated into other Rust projects.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
wallet-utils = "0.1.0"
```

# Usage

```rust

use std::error::Error;
use wallet_utils::decode_calldata;

fn main() -> Result<(), Box<dyn Error>> {
let calldata = "b61d27f6000000000000000000000000b382daf7230b73f71a766c96707cf9ec6316b360000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000144e555c489000000000000000000000000efb80041d435d26d397bba4d4138b8232ea82d5400000000000000000000000081cc3c9c23ba6ce4dcf10b116079710f15336fd30000000000000000000000000000000000000000000035c900000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000009581cc3c9c23ba6ce4dcf10b116079710f15336fd30000000000000000000000000000000000000000000000000000000066c97cdd0000000000000000000000000000000000000000000000000000000066c975d5d78c691a386ed33f24c0611b5021cd9a5231e763bc1127a66840231d97f599a7792bc42640ca19bf7c6f1d04a27095decf66a8dd13ea52f0c221e821965004981c000000000000000000000000000000000000000000000000000000000000000000000000000000";

    let (target, value, calldata) = decode_calldata(calldata)?;

    println!("Decoded Target: {}", target);
    println!("Decoded Value: {}", value);
    println!("Decoded Calldata: {}", calldata);

    Ok(())

}

```

# Running Tests

To run the tests for the library:

```sh

cargo test

```

# Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

# License

This project is licensed under the MIT License.

```

```
