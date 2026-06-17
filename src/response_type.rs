use std::fmt;

use crate::value_object::ValueEnum;

/// OAuth 2.0 response type defined in RFC 6749.
///
/// Used as the `response_type` parameter in requests to the authorization
/// endpoint. The [`Display`] implementation returns the RFC wire-format string.
///
/// | Variant | Wire Value | Purpose |
/// |---|---|---|
/// | `Code` | `code` | Authorization code flow (RFC 6749 Section 4.1) |
/// | `Token` | `token` | Implicit flow (RFC 6749 Section 4.2) |
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseType {
    /// Response type for the authorization code flow (`code`).
    Code,
    /// Response type for the implicit flow (`token`).
    Token,
}

impl ValueEnum for ResponseType {}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseType::Code => write!(f, "code"),
            ResponseType::Token => write!(f, "token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variants() {
        assert_eq!(ResponseType::Code.to_string(), "code");
        assert_eq!(ResponseType::Token.to_string(), "token");
    }

    #[test]
    fn equal_variants_are_equal() {
        assert_eq!(ResponseType::Code, ResponseType::Code);
        assert_eq!(ResponseType::Token, ResponseType::Token);
    }

    #[test]
    fn different_variants_are_not_equal() {
        assert_ne!(ResponseType::Code, ResponseType::Token);
    }

    #[test]
    fn clone_produces_equal_value() {
        let original = ResponseType::Code;
        assert_eq!(original, original.clone());
    }
}
