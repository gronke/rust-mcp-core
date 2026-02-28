//! Token generation utilities.

use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a random 32-character hex token.
///
/// Uses timestamp and process ID for randomness. Suitable for
/// generating API tokens that need to be unique but don't require
/// cryptographic security.
///
/// # Example
///
/// ```rust
/// use mcp_core::generate_random_token;
///
/// let token = generate_random_token();
/// assert_eq!(token.len(), 32);
/// assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
/// ```
pub fn generate_random_token() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    // Simple random generation using timestamp and process id
    let pid = std::process::id();
    let random: u64 = (timestamp as u64)
        .wrapping_mul(pid as u64)
        .wrapping_add(0xdeadbeef);
    format!("{:016x}{:016x}", timestamp as u64, random)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_length() {
        let token = generate_random_token();
        assert_eq!(token.len(), 32);
    }

    #[test]
    fn test_token_is_hex() {
        let token = generate_random_token();
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_tokens_are_unique() {
        let token1 = generate_random_token();
        // Small delay to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(1));
        let token2 = generate_random_token();
        assert_ne!(token1, token2);
    }
}
