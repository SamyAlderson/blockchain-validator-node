//! Network communication utilities
//!
//! This module provides functions for sending and receiving network messages.

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use tokio::net::TcpStream as AsyncTcpStream;
use serde_json::json;

/// Establishes a TCP listener at the specified address and port.
///
/// The `addr` parameter should be in the format "0.0.0.0:8080", where "0.0.0.0" is the IP address
/// to bind to and "8080" is the port number.
///
/// Returns an `AsyncTcpListener` that can be used to accept incoming connections.
async fn listen_for_connections(addr: &str) -> Result<AsyncTcpListener, std::io::Error> {
    let listener = TcpListener::bind(addr).await?;
    Ok(listener)
}

/// Sends a message over a TCP connection.
///
/// The `message` parameter is the message to be sent, which should be a JSON serializable payload.
///
/// Returns `Ok` if the message was sent successfully, or an `Error` if an IO error occurred.
async fn send_message(stream: &mut AsyncTcpStream, message: serde_json::Value) -> Result<(), std::io::Error> {
    let serialized_message = json!(message);
    stream.write_all(serialized_message.to_string().as_bytes()).await?;
    Ok(())
}

/// Receives a message from a TCP connection.
///
/// Returns the received message as a JSON serializable payload, or an `Error` if an IO error occurred.
async fn receive_message(stream: &mut AsyncTcpStream) -> Result<serde_json::Value, std::io::Error> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    let message = serde_json::from_slice(&buffer)?;
    Ok(message)
}

/// Establishes a TCP connection to the specified address and port.
///
/// The `addr` parameter should be in the format "0.0.0.0:8080", where "0.0.0.0" is the IP address
/// to connect to and "8080" is the port number.
///
/// Returns a `TcpStream` that can be used to send and receive messages.
async fn connect_to_server(addr: &str) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(addr).await
}

/// Sends a message to a server.
///
/// The `message` parameter is the message to be sent, which should be a JSON serializable payload.
///
/// Returns `Ok` if the message was sent successfully, or an `Error` if an IO error occurred.
async fn send_message_to_server(addr: &str, message: serde_json::Value) -> Result<(), std::io::Error> {
    let mut stream = connect_to_server(addr).await?;
    send_message(&mut stream, message).await?;
    Ok(())
}

/// Receives a message from a server.
///
/// Returns the received message as a JSON serializable payload, or an `Error` if an IO error occurred.
async fn receive_message_from_server(addr: &str) -> Result<serde_json::Value, std::io::Error> {
    let mut stream = connect_to_server(addr).await?;
    receive_message(&mut stream).await
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        let message = json!({"key": "value"});
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        send_message(&mut stream, message).await.unwrap();
    }

    #[tokio::test]
    async fn test_receive_message() {
        let message = json!({"key": "value"});
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        stream.write_all(message.to_string().as_bytes()).await.unwrap();
        let received_message = receive_message(&mut stream).await.unwrap();
        assert_eq!(received_message, message);
    }
}