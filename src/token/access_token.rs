use crate::secret::Secret;
use crate::value_object::{SecretValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`AccessToken`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessTokenError {
    /// The access token is empty or whitespace only.
    Empty,
}

impl ValueObjectError for AccessTokenError {}

impl fmt::Display for AccessTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessTokenError::Empty => write!(f, "access_token must not be empty"),
        }
    }
}

impl std::error::Error for AccessTokenError {}

/// Secret value object representing an OAuth 2.0 access token (RFC 6749 Section 1.4).
///
/// Credential used to access protected resources. Wrapped in [`Secret<String>`],
/// so `Debug` and `Display` output `"[REDACTED]"`. Use [`expose_secret`] to
/// extract the value.
///
/// [`expose_secret`]: crate::value_object::SecretValueObject::expose_secret
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken(Secret<String>);

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl SecretValueObject for AccessToken {
    type Value = String;
    type Error = AccessTokenError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        Self::is_valid(&value)?;
        Ok(Self(Secret::new(value.trim().to_string())))
    }

    fn expose_secret(&self) -> &Self::Value {
        self.0.expose_secret()
    }

    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error> {
        Self::is_valid(&value)?;
        self.0 = Secret::new(value.trim().to_string());
        Ok(())
    }

    fn is_valid(value: &Self::Value) -> Result<(), Self::Error> {
        if value.trim().is_empty() {
            return Err(AccessTokenError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let token = AccessToken::new("super-token-value".to_string()).unwrap();
        assert_eq!(token.expose_secret(), "super-token-value");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(
            AccessToken::new("".to_string()),
            Err(AccessTokenError::Empty)
        );
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(
            AccessToken::new("   ".to_string()),
            Err(AccessTokenError::Empty)
        );
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        let token = AccessToken::new("  abc  ".to_string()).unwrap();
        assert_eq!(token.expose_secret(), "abc");
    }

    #[test]
    fn set_value_updates_on_success() {
        let mut token = AccessToken::new("a".to_string()).unwrap();
        token.set_value("b".to_string()).unwrap();
        assert_eq!(token.expose_secret(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut token = AccessToken::new("a".to_string()).unwrap();
        assert_eq!(
            token.set_value("   ".to_string()),
            Err(AccessTokenError::Empty)
        );
        assert_eq!(token.expose_secret(), "a");
    }

    #[test]
    fn display_does_not_leak_secret() {
        let token = AccessToken::new("super-token-value".to_string()).unwrap();
        let displayed = token.to_string();
        assert_eq!(displayed, "[REDACTED]");
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let token = AccessToken::new("super-token-value".to_string()).unwrap();
        let debugged = format!("{:?}", token);
        assert!(!debugged.contains("super-token-value"));
    }

    #[test]
    fn equal_values_are_equal() {
        let a = AccessToken::new("same-value".to_string()).unwrap();
        let b = AccessToken::new("same-value".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_values_are_not_equal() {
        let a = AccessToken::new("value-a".to_string()).unwrap();
        let b = AccessToken::new("value-b".to_string()).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = AccessToken::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "access_token must not be empty");
    }
}
