// Utility functions for the validator node
// These functions are not critical to the blockchain validation process but are
// useful for debugging and logging purposes

use log::{error, info};
use serde_json::json;
use std::collections::HashMap;

/// Convert a hexadecimal string to a `u64` integer
///
/// This function is used to convert the hash of a block to a numeric value
/// that can be compared with other blocks.
///
/// # Errors
///
/// Returns an error if the input string is not a valid hexadecimal number.
fn hex_to_u64(hex: &str) -> Result<u64, String> {
    match u64::from_str_radix(hex, 16) {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Invalid hexadecimal string: {}", hex)),
    }
}

/// Convert a `u64` integer to a hexadecimal string
///
/// This function is used to convert the hash of a block to a hexadecimal string
/// that can be stored in a file or sent over the network.
fn u64_to_hex(num: u64) -> String {
    format!("{:x}", num)
}

/// Get the last `n` elements from a vector
///
/// This function is used to get the last `n` blocks from the blockchain.
fn get_last_n_elements<T>(vec: &Vec<T>, n: usize) -> Vec<T> {
    if n > vec.len() {
        return vec.clone();
    }
    vec.split_at(vec.len() - n).1.to_vec()
}

/// Merge two `HashMap`s into a single `HashMap`
///
/// This function is used to merge the data from two blocks into a single block.
fn merge_hashmaps<T>(left: &HashMap<String, T>, right: &HashMap<String, T>) -> HashMap<String, T> {
    let mut merged = left.clone();
    for (key, value) in right {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

/// Debugging function to print information about a block
///
/// This function is used to print information about a block, such as its hash and
/// the hashes of its transactions.
fn print_block_info(block: &HashMap<String, String>) {
    info!("Block Info:");
    info!("Hash: {}", block.get("hash").unwrap());
    info!("Transactions:");
    for (key, value) in block.get("transactions").unwrap().as_str().split(",") {
        info!("  {}", key.trim().to_string());
    }
}