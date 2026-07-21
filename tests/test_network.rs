// Network utilities unit tests
#[cfg(test)]
mod test_network {
    use super::*;
    use crate::network::{Network, NetworkError};
    use crate::utils::random_string;
    use rand::Rng;
    use serde_json::{json, Value};
    use tokio;

    #[test]
    fn test_network_connect() -> Result<(), NetworkError> {
        // Create a test network with a single node
        let network = Network::new(vec![random_string(16).into()])?;

        // Connect to the network
        network.connect()?;

        // Verify that the connection was successful
        assert!(network.is_connected());

        Ok(())
    }

    #[test]
    fn test_network_disconnect() -> Result<(), NetworkError> {
        // Create a test network with a single node
        let network = Network::new(vec![random_string(16).into()])?;

        // Connect to the network
        network.connect()?;

        // Disconnect from the network
        network.disconnect()?;

        // Verify that the connection was lost
        assert!(!network.is_connected());

        Ok(())
    }

    #[test]
    fn test_network_send_receive() -> Result<(), NetworkError> {
        // Create a test network with two nodes
        let network = Network::new(vec![random_string(16).into(), random_string(16).into()])?;

        // Connect to the network
        network.connect()?;

        // Send a message from one node to the other
        let message = json!({"foo": "bar"});
        network.send_message(0, 1, message)?;

        // Receive the message on the other node
        let received_message = network.receive_message(1)?;

        // Verify that the message was received correctly
        assert_eq!(received_message, message);

        Ok(())
    }

    #[test]
    fn test_network_error_handling() -> Result<(), NetworkError> {
        // Create a test network with a single node
        let network = Network::new(vec![random_string(16).into()])?;

        // Simulate a network error by setting the connection to false
        network.set_connected(false);

        // Attempt to send a message through the network
        let message = json!({"foo": "bar"});
        let result = network.send_message(0, 0, message);

        // Verify that the error is handled correctly
        match result {
            Err(NetworkError::ConnectionError) => (),
            _ => panic!("Expected a connection error"),
        }

        Ok(())
    }
}