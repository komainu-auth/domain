use std::fmt;

use crate::value_object::ValueEnum;

/// OAuth 2.0 grant type defined in RFC 6749.
///
/// Used as the `grant_type` parameter in requests to the token endpoint.
/// The [`Display`] implementation returns the RFC wire-format string.
///
/// # Variants
///
/// | Variant | Wire Value | RFC Section |
/// |---|---|---|
/// | `AuthorizationCode` | `authorization_code` | 4.1 |
/// | `Password` | `password` | 4.3 |
/// | `ClientCredentials` | `client_credentials` | 4.4 |
/// | `RefreshToken` | `refresh_token` | 6 |
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrantType {
    /// Authorization code grant (RFC 6749 Section 4.1).
    AuthorizationCode,
    /// Resource owner password credentials grant (RFC 6749 Section 4.3).
    Password,
    /// Client credentials grant (RFC 6749 Section 4.4).
    ClientCredentials,
    /// Refresh token grant (RFC 6749 Section 6).
    RefreshToken,
}

impl ValueEnum for GrantType {}

impl fmt::Display for GrantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrantType::AuthorizationCode => write!(f, "authorization_code"),
            GrantType::Password => write!(f, "password"),
            GrantType::ClientCredentials => write!(f, "client_credentials"),
            GrantType::RefreshToken => write!(f, "refresh_token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variants() {
        assert_eq!(
            GrantType::AuthorizationCode.to_string(),
            "authorization_code"
        );
        assert_eq!(GrantType::Password.to_string(), "password");
        assert_eq!(
            GrantType::ClientCredentials.to_string(),
            "client_credentials"
        );
        assert_eq!(GrantType::RefreshToken.to_string(), "refresh_token");
    }

    #[test]
    fn equal_variants_are_equal() {
        assert_eq!(GrantType::AuthorizationCode, GrantType::AuthorizationCode);
        assert_eq!(GrantType::Password, GrantType::Password);
        assert_eq!(GrantType::ClientCredentials, GrantType::ClientCredentials);
        assert_eq!(GrantType::RefreshToken, GrantType::RefreshToken);
    }

    #[test]
    fn different_variants_are_not_equal() {
        assert_ne!(GrantType::AuthorizationCode, GrantType::Password);
        assert_ne!(GrantType::ClientCredentials, GrantType::RefreshToken);
    }

    #[test]
    fn clone_produces_equal_value() {
        let original = GrantType::ClientCredentials;
        assert_eq!(original, original.clone());
    }
}
