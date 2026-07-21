//! Blockchain validator implementation
//!
//! This module contains the logic for validating blocks in the blockchain.

use rand::Rng;
use serde::{Serialize, Deserialize};
use log::{debug, info};

pub struct Validator {
    /// Random number generator for shuffling blocks
    rng: rand::ThreadRng,
}

impl Validator {
    /// Create a new instance of the validator
    pub fn new() -> Self {
        Validator {
            rng: rand::thread_rng(),
        }
    }

    /// Validate a block in the blockchain
    pub fn validate_block(&mut self, block: Block) -> Result<(), ValidatorError> {
        // Check if the block has a valid hash
        if block.hash == "" {
            return Err(ValidatorError::InvalidHash);
        }

        // Check if the block has a valid previous hash
        if block.prev_hash == "" {
            return Err(ValidatorError::InvalidPreviousHash);
        }

        // Check if the block has a valid timestamp
        if block.timestamp < 0 {
            return Err(ValidatorError::InvalidTimestamp);
        }

        // Check if the block has a valid data
        if block.data.is_empty() {
            return Err(ValidatorError::InvalidData);
        }

        // If all checks pass, return Ok
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    /// Hash of the block
    pub hash: String,
    /// Hash of the previous block
    pub prev_hash: String,
    /// Timestamp of the block
    pub timestamp: u64,
    /// Data of the block
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum ValidatorError {
    InvalidHash,
    InvalidPreviousHash,
    InvalidTimestamp,
    InvalidData,
}

impl std::error::Error for ValidatorError {}

impl std::fmt::Display for ValidatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidatorError::InvalidHash => write!(f, "Invalid block hash"),
            ValidatorError::InvalidPreviousHash => write!(f, "Invalid previous block hash"),
            ValidatorError::InvalidTimestamp => write!(f, "Invalid block timestamp"),
            ValidatorError::InvalidData => write!(f, "Invalid block data"),
        }
    }
}