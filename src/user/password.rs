use crate::secret::Secret;
use crate::value_object::{SecretValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`Password`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordError {
    /// The password is empty or whitespace only.
    Empty,
}

impl ValueObjectError for PasswordError {}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::Empty => write!(f, "password must not be empty"),
        }
    }
}

impl std::error::Error for PasswordError {}

/// Secret value object holding a plaintext password.
///
/// Holds the raw password supplied by a user (e.g. from a sign-up or login
/// request) before it is hashed into a [`PasswordHash`]. Wrapped in
/// [`Secret<String>`], so `Debug` and `Display` output `"[REDACTED]"`. Use
/// [`expose_secret`] to extract the password value.
///
/// [`PasswordHash`]: crate::user::PasswordHash
/// [`expose_secret`]: crate::value_object::SecretValueObject::expose_secret
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(Secret<String>);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl SecretValueObject for Password {
    type Value = String;
    type Error = PasswordError;

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
            return Err(PasswordError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let secret = Password::new("super-secret-value".to_string()).unwrap();
        assert_eq!(secret.expose_secret(), "super-secret-value");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(
            Password::new("".to_string()),
            Err(PasswordError::Empty)
        );
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(
            Password::new("   ".to_string()),
            Err(PasswordError::Empty)
        );
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        let secret = Password::new("  abc  ".to_string()).unwrap();
        assert_eq!(secret.expose_secret(), "abc");
    }

    #[test]
    fn set_value_updates_on_success() {
        let mut secret = Password::new("a".to_string()).unwrap();
        secret.set_value("b".to_string()).unwrap();
        assert_eq!(secret.expose_secret(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut secret = Password::new("a".to_string()).unwrap();
        assert_eq!(
            secret.set_value("   ".to_string()),
            Err(PasswordError::Empty)
        );
        assert_eq!(secret.expose_secret(), "a");
    }

    #[test]
    fn display_does_not_leak_secret() {
        let secret = Password::new("super-secret-value".to_string()).unwrap();
        let displayed = secret.to_string();
        assert_eq!(displayed, "[REDACTED]");
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let secret = Password::new("super-secret-value".to_string()).unwrap();
        let debugged = format!("{:?}", secret);
        assert!(!debugged.contains("super-secret-value"));
    }

    #[test]
    fn equal_values_are_equal() {
        let a = Password::new("same-value".to_string()).unwrap();
        let b = Password::new("same-value".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_values_are_not_equal() {
        let a = Password::new("value-a".to_string()).unwrap();
        let b = Password::new("value-b".to_string()).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = Password::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "password must not be empty");
    }
}
