//! Main entry point for the validator node

use std::time::Instant;
use std::env;
use std::error::Error;
use log::{info, debug, error};
use tokio::select;
use tokio::time::{sleep, Duration};
use rand::Rng;
use serde_json::json;

mod validator;
mod network;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();

    // Initialize validator and network instances
    let validator = validator::Validator::new()?;
    let network = network::Network::new()?;

    // Spawn validator and network tasks
    let validator_task = validator.run().await?;
    let network_task = network.run().await?;

    // Wait for tasks to complete
    select! {
        _ = validator_task => {
            error!("Validator task failed: {}", validator_task);
        }
        _ = network_task => {
            error!("Network task failed: {}", network_task);
        }
    };

    // Wait for a short period to allow tasks to finish
    sleep(Duration::from_secs(1)).await;

    // Print final status
    info!("Validator node shutting down...");
    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run");
}

fn print_help() {
    println!("Help message:");
    println!("  cargo run - Start the validator node");
}

fn print_version() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}