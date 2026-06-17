use crate::value_object::{ValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`ClientId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientIdError {
    /// The client ID is empty or whitespace only.
    Empty,
}

impl ValueObjectError for ClientIdError {}

impl std::fmt::Display for ClientIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientIdError::Empty => write!(f, "client_id must not be empty"),
        }
    }
}

impl std::error::Error for ClientIdError {}

/// Value object representing an OAuth 2.0 client identifier (RFC 6749 Section 2.2).
///
/// Rejects empty or whitespace-only values at construction time. Leading and
/// trailing whitespace is trimmed automatically.
///
/// # Examples
///
/// ```rust,ignore
/// use domain::client::ClientId;
/// use domain::value_object::ValueObject;
///
/// let id = ClientId::new("my-client-id".to_string()).unwrap();
/// assert_eq!(id.value(), "my-client-id");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientId(String);

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for ClientId {
    type Value = String;
    type Error = ClientIdError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        Self::is_valid(&value)?;
        Ok(Self(value.trim().to_string()))
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }

    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error> {
        Self::is_valid(&value)?;
        self.0 = value.trim().to_string();
        Ok(())
    }

    fn is_valid(value: &Self::Value) -> Result<(), Self::Error> {
        if value.trim().is_empty() {
            return Err(ClientIdError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let id = ClientId::new("client-123".to_string()).unwrap();
        assert_eq!(id.value(), "client-123");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(ClientId::new("".to_string()), Err(ClientIdError::Empty));
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(ClientId::new("   ".to_string()), Err(ClientIdError::Empty));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_on_construction() {
        // 仕様変更: 前後の空白は除去されて格納される
        let id = ClientId::new(" abc ".to_string()).unwrap();
        assert_eq!(id.value(), "abc");
    }

    #[test]
    fn set_value_trims_and_updates() {
        let mut id = ClientId::new("a".to_string()).unwrap();
        id.set_value("  b  ".to_string()).unwrap();
        assert_eq!(id.value(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut id = ClientId::new("a".to_string()).unwrap();
        assert_eq!(id.set_value("   ".to_string()), Err(ClientIdError::Empty));
        assert_eq!(id.value(), "a"); // 元の値が保持される
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = ClientId::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "client_id must not be empty");
    }

    #[test]
    fn equal_values_are_equal() {
        let a = ClientId::new("x".to_string()).unwrap();
        let b = ClientId::new("x".to_string()).unwrap();
        assert_eq!(a, b);
    }
}
