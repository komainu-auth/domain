use std::fmt;

use crate::value_object::ValueEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrantType {
    AuthorizationCode,
    Password,
    ClientCredentials,
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
