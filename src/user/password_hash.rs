use crate::secret::Secret;
use crate::value_object::{SecretValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`PasswordHash`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHashError {
    /// The password hash is empty or whitespace only.
    Empty,
}

impl ValueObjectError for PasswordHashError {}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordHashError::Empty => write!(f, "password_hash must not be empty"),
        }
    }
}

impl std::error::Error for PasswordHashError {}

/// Secret value object holding a password hash.
///
/// Stores a hashed string (e.g. from `bcrypt` or `argon2`), not the plaintext
/// password. Wrapped in [`Secret<String>`], so `Debug` and `Display` output
/// `"[REDACTED]"`. Use [`expose_secret`] to extract the hash value.
///
/// [`expose_secret`]: crate::value_object::SecretValueObject::expose_secret
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(Secret<String>);

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl SecretValueObject for PasswordHash {
    type Value = String;
    type Error = PasswordHashError;

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
            return Err(PasswordHashError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let secret = PasswordHash::new("super-secret-value".to_string()).unwrap();
        assert_eq!(secret.expose_secret(), "super-secret-value");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(
            PasswordHash::new("".to_string()),
            Err(PasswordHashError::Empty)
        );
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(
            PasswordHash::new("   ".to_string()),
            Err(PasswordHashError::Empty)
        );
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        let secret = PasswordHash::new("  abc  ".to_string()).unwrap();
        assert_eq!(secret.expose_secret(), "abc");
    }

    #[test]
    fn set_value_updates_on_success() {
        let mut secret = PasswordHash::new("a".to_string()).unwrap();
        secret.set_value("b".to_string()).unwrap();
        assert_eq!(secret.expose_secret(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut secret = PasswordHash::new("a".to_string()).unwrap();
        assert_eq!(
            secret.set_value("   ".to_string()),
            Err(PasswordHashError::Empty)
        );
        assert_eq!(secret.expose_secret(), "a");
    }

    #[test]
    fn display_does_not_leak_secret() {
        let secret = PasswordHash::new("super-secret-value".to_string()).unwrap();
        let displayed = secret.to_string();
        assert_eq!(displayed, "[REDACTED]");
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let secret = PasswordHash::new("super-secret-value".to_string()).unwrap();
        let debugged = format!("{:?}", secret);
        assert!(!debugged.contains("super-secret-value"));
    }

    #[test]
    fn equal_values_are_equal() {
        let a = PasswordHash::new("same-value".to_string()).unwrap();
        let b = PasswordHash::new("same-value".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_values_are_not_equal() {
        let a = PasswordHash::new("value-a".to_string()).unwrap();
        let b = PasswordHash::new("value-b".to_string()).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = PasswordHash::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "password_hash must not be empty");
    }
}
