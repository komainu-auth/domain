use crate::secret::Secret;
use crate::value_object::{SecretValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`AuthorizationCode`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationCodeError {
    /// The authorization code is empty or whitespace only.
    Empty,
}

impl ValueObjectError for AuthorizationCodeError {}

impl fmt::Display for AuthorizationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthorizationCodeError::Empty => write!(f, "authorization_code must not be empty"),
        }
    }
}

impl std::error::Error for AuthorizationCodeError {}

/// Secret value object representing an OAuth 2.0 authorization code (RFC 6749 Section 4.1.2).
///
/// A short-lived token issued by the authorization endpoint and used in requests
/// to the token endpoint.
///
/// Wrapped in [`Secret<String>`], so `Debug` and `Display` output `"[REDACTED]"`.
/// Use [`expose_secret`] to extract the value.
///
/// [`expose_secret`]: crate::value_object::SecretValueObject::expose_secret
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationCode(Secret<String>);

impl fmt::Display for AuthorizationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl SecretValueObject for AuthorizationCode {
    type Value = String;
    type Error = AuthorizationCodeError;

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
            return Err(AuthorizationCodeError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let code = AuthorizationCode::new("super-code-value".to_string()).unwrap();
        assert_eq!(code.expose_secret(), "super-code-value");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(
            AuthorizationCode::new("".to_string()),
            Err(AuthorizationCodeError::Empty)
        );
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(
            AuthorizationCode::new("   ".to_string()),
            Err(AuthorizationCodeError::Empty)
        );
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        let code = AuthorizationCode::new("  abc  ".to_string()).unwrap();
        assert_eq!(code.expose_secret(), "abc");
    }

    #[test]
    fn set_value_updates_on_success() {
        let mut code = AuthorizationCode::new("a".to_string()).unwrap();
        code.set_value("b".to_string()).unwrap();
        assert_eq!(code.expose_secret(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut code = AuthorizationCode::new("a".to_string()).unwrap();
        assert_eq!(
            code.set_value("   ".to_string()),
            Err(AuthorizationCodeError::Empty)
        );
        assert_eq!(code.expose_secret(), "a");
    }

    #[test]
    fn display_does_not_leak_secret() {
        let code = AuthorizationCode::new("super-code-value".to_string()).unwrap();
        let displayed = code.to_string();
        assert_eq!(displayed, "[REDACTED]");
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let code = AuthorizationCode::new("super-code-value".to_string()).unwrap();
        let debugged = format!("{:?}", code);
        assert!(!debugged.contains("super-code-value"));
    }

    #[test]
    fn equal_values_are_equal() {
        let a = AuthorizationCode::new("same-value".to_string()).unwrap();
        let b = AuthorizationCode::new("same-value".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_values_are_not_equal() {
        let a = AuthorizationCode::new("value-a".to_string()).unwrap();
        let b = AuthorizationCode::new("value-b".to_string()).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = AuthorizationCode::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "authorization_code must not be empty");
    }
}
