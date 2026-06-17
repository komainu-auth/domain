use std::fmt;

use crate::value_object::ValueEnum;

/// OAuth 2.0 token type (RFC 6750).
///
/// Used in the `token_type` field of token endpoint responses. Currently only
/// `Bearer` tokens are supported.
///
/// # Specification
///
/// - RFC 6750: The OAuth 2.0 Authorization Framework: Bearer Token Usage
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Bearer token (RFC 6750). The most widely used token type.
    Bearer,
}

impl ValueEnum for TokenType {}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Bearer => write!(f, "Bearer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variants() {
        assert_eq!(TokenType::Bearer.to_string(), "Bearer");
    }

    #[test]
    fn equal_variants_are_equal() {
        assert_eq!(TokenType::Bearer, TokenType::Bearer);
    }

    #[test]
    fn clone_produces_equal_value() {
        let original = TokenType::Bearer;
        assert_eq!(original, original.clone());
    }
}
