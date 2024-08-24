use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolType};
use hex::decode;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct FunctionInfo {
    abi: Vec<String>,
    format: Vec<String>,
    r#type: String,
}

type FunctionSelectorMap = HashMap<String, FunctionInfo>;

pub fn decode_calldata(data: &str) -> Result<(String, String, String), Box<dyn Error>> {
    // Load the JSON file
    let file_content = fs::read_to_string("./src/wallet-sig-map.json")?;
    let selectors: FunctionSelectorMap = serde_json::from_str(&file_content)?;

    // Convert the hex string to bytes
    let data_bytes = decode(data)?;

    // Extract the method selector (first 4 bytes)
    let method_selector = &data_bytes[..4];
    let selector_str = format!("0x{}", hex::encode(method_selector));

    // Check if the method selector matches one of the known selectors
    if let Some(function_info) = selectors.get(&selector_str) {
        // Handle the "simple" type by default
        if function_info.r#type == "simple"
            && function_info.abi == vec!["address", "uint256", "bytes"]
        {
            // Define the ABI type structure
            type MyTuple = sol!((address, uint256, bytes));

            // Decode the ABI data
            let param_bytes = &data_bytes[4..];
            let decoded = MyTuple::abi_decode_params(param_bytes, false)?;

            // Extract the decoded values
            let target = decoded.0.to_string();
            let value = decoded.1.to_string();
            let calldata = decoded.2.to_string();

            // Return the target (method selector), value, and calldata as strings
            return Ok((target, value, calldata));
        }
        // Handle other types or more complex structures here if needed...
    }

    // If no match is found, return an error
    Err("Unsupported method selector or ABI structure".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_abi() {
        let calldata = "b61d27f6000000000000000000000000b382daf7230b73f71a766c96707cf9ec6316b360000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000144e555c489000000000000000000000000efb80041d435d26d397bba4d4138b8232ea82d5400000000000000000000000081cc3c9c23ba6ce4dcf10b116079710f15336fd30000000000000000000000000000000000000000000035c900000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000009581cc3c9c23ba6ce4dcf10b116079710f15336fd30000000000000000000000000000000000000000000000000000000066c97cdd0000000000000000000000000000000000000000000000000000000066c975d5d78c691a386ed33f24c0611b5021cd9a5231e763bc1127a66840231d97f599a7792bc42640ca19bf7c6f1d04a27095decf66a8dd13ea52f0c221e821965004981c000000000000000000000000000000000000000000000000000000000000000000000000000000";

        let (target, address, bytes_data) = decode_calldata(calldata).unwrap();

        assert_eq!(
            target.to_ascii_lowercase(),
            "0xb382daf7230b73f71a766c96707cf9ec6316b360"
        );
        assert_eq!(address, "0");
        assert!(bytes_data.to_ascii_lowercase().contains("0xe555c489"));
    }
}
