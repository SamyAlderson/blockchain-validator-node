// tests/test_validator.rs
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use serde_json::json;
    use std::collections::HashMap;
    use validator::Validator;

    #[test]
    fn test_validator_invalid() {
        let validator = Validator::new();
        let mut invalid_block = HashMap::new();
        invalid_block.insert("header".to_string(), json!({}));
        assert!(validator.validate_block(&invalid_block).is_err());
    }

    #[test]
    fn test_validator_valid() {
        let validator = Validator::new();
        let mut valid_block = HashMap::new();
        valid_block.insert("header".to_string(), json!({
            "version": 1,
            "prev_hash": "some prev hash".to_string(),
            "merkle_root": "some merkle root".to_string(),
            "timestamp": 1643723900,
            "difficulty": 2,
            "nonce": 42,
        }));
        valid_block.insert("txs".to_string(), json!([1, 2, 3]));
        assert!(validator.validate_block(&valid_block).is_ok());
    }

    #[test]
    fn test_validator_invalid_timestamp() {
        let validator = Validator::new();
        let mut invalid_block = HashMap::new();
        invalid_block.insert("header".to_string(), json!({
            "version": 1,
            "prev_hash": "some prev hash".to_string(),
            "merkle_root": "some merkle root".to_string(),
            "timestamp": 16437239000,
            "difficulty": 2,
            "nonce": 42,
        }));
        assert!(validator.validate_block(&invalid_block).is_err());
    }

    #[test]
    fn test_validator_invalid_difficulty() {
        let validator = Validator::new();
        let mut invalid_block = HashMap::new();
        invalid_block.insert("header".to_string(), json!({
            "version": 1,
            "prev_hash": "some prev hash".to_string(),
            "merkle_root": "some merkle root".to_string(),
            "timestamp": 1643723900,
            "difficulty": -2,
            "nonce": 42,
        }));
        assert!(validator.validate_block(&invalid_block).is_err());
    }

    #[test]
    fn test_validator_valid_merkle_root() {
        let validator = Validator::new();
        let mut valid_block = HashMap::new();
        valid_block.insert("header".to_string(), json!({
            "version": 1,
            "prev_hash": "some prev hash".to_string(),
            "merkle_root": "some merkle root".to_string(),
            "timestamp": 1643723900,
            "difficulty": 2,
            "nonce": 42,
        }));
        valid_block.insert("txs".to_string(), json!([1, 2, 3]));
        let hash = validator.hash_txs(&valid_block["txs"]);
        valid_block.insert("merkle_root".to_string(), json!(hash));
        assert!(validator.validate_block(&valid_block).is_ok());
    }
}