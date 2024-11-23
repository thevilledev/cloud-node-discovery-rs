use crate::DiscoveryError;
use std::collections::HashMap;

pub fn parse_config(config: &str) -> Result<HashMap<String, String>, DiscoveryError> {
    let mut result = HashMap::new();

    for pair in config.split(',') {
        let mut parts = pair.split('=');
        let key = parts
            .next()
            .ok_or_else(|| DiscoveryError::ConfigError("Missing key".to_string()))?
            .trim();
        let value = parts
            .next()
            .ok_or_else(|| DiscoveryError::ConfigError("Missing value".to_string()))?
            .trim();

        if parts.next().is_some() {
            return Err(DiscoveryError::ConfigError(format!(
                "Invalid format in pair: {}",
                pair
            )));
        }

        if key.is_empty() {
            return Err(DiscoveryError::ConfigError("Empty key".to_string()));
        }

        if value.is_empty() {
            return Err(DiscoveryError::ConfigError("Empty value".to_string()));
        }

        if result.insert(key.to_string(), value.to_string()).is_some() {
            return Err(DiscoveryError::ConfigError(format!(
                "Duplicate key: {}",
                key
            )));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = "region=us-east-1,tag_key=foo,tag_value=bar";
        let result = parse_config(config).unwrap();

        assert_eq!(result.get("region"), Some(&"us-east-1".to_string()));
        assert_eq!(result.get("tag_key"), Some(&"foo".to_string()));
        assert_eq!(result.get("tag_value"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_parse_config_errors() {
        // Missing value
        assert!(parse_config("key=").is_err());

        // Missing key
        assert!(parse_config("=value").is_err());

        // Invalid format
        assert!(parse_config("key=value=extra").is_err());

        // Duplicate key
        assert!(parse_config("key=value1,key=value2").is_err());
    }
}
