use std::fmt;

use crate::value_object::ValueEnum;

/// OAuth 2.0 client type defined in RFC 6749 Section 2.1.
///
/// Distinguishes whether a client can securely hold a secret.
///
/// | Variant | Wire Value | Description |
/// |---|---|---|
/// | `Confidential` | `confidential` | Clients that can securely hold a secret (e.g. server-side apps) |
/// | `Public` | `public` | Clients that cannot securely hold a secret (e.g. SPAs, native apps) |
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientType {
    /// A client that cannot securely hold a secret (e.g. SPA, mobile app).
    Public,
    /// A client that can securely hold a secret (e.g. server-side app).
    Confidential,
}

impl ValueEnum for ClientType {}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientType::Public => write!(f, "public"),
            ClientType::Confidential => write!(f, "confidential"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variants() {
        assert_eq!(ClientType::Public.to_string(), "public");
        assert_eq!(ClientType::Confidential.to_string(), "confidential");
    }

    #[test]
    fn equal_variants_are_equal() {
        assert_eq!(ClientType::Public, ClientType::Public);
        assert_eq!(ClientType::Confidential, ClientType::Confidential);
    }

    #[test]
    fn different_variants_are_not_equal() {
        assert_ne!(ClientType::Public, ClientType::Confidential);
    }

    #[test]
    fn clone_produces_equal_value() {
        let original = ClientType::Confidential;
        assert_eq!(original, original.clone());
    }
}
