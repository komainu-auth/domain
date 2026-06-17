use crate::value_object::{ValueObject, ValueObjectError};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserIdError {
    Empty,
}

impl ValueObjectError for UserIdError {}

impl std::fmt::Display for UserIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserIdError::Empty => write!(f, "user_id must not be empty"),
        }
    }
}

impl std::error::Error for UserIdError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for UserId {
    type Value = String;
    type Error = UserIdError;

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
            return Err(UserIdError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let user_id: UserId = UserId::new("user_id-123".to_string()).unwrap();
        assert_eq!(user_id.value(), "user_id-123");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(UserId::new("".to_string()), Err(UserIdError::Empty));
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(UserId::new("   ".to_string()), Err(UserIdError::Empty));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_on_construction() {
        // 仕様変更: 前後の空白は除去されて格納される
        let user_id = UserId::new(" abc ".to_string()).unwrap();
        assert_eq!(user_id.value(), "abc");
    }

    #[test]
    fn set_value_trims_and_updates() {
        let mut user_id = UserId::new("a".to_string()).unwrap();
        user_id.set_value("  b  ".to_string()).unwrap();
        assert_eq!(user_id.value(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut user_id = UserId::new("a".to_string()).unwrap();
        assert_eq!(user_id.set_value("   ".to_string()), Err(UserIdError::Empty));
        assert_eq!(user_id.value(), "a"); // 元の値が保持される
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = UserId::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "user_id must not be empty");
    }

    #[test]
    fn equal_values_are_equal() {
        let a = UserId::new("x".to_string()).unwrap();
        let b = UserId::new("x".to_string()).unwrap();
        assert_eq!(a, b);
    }
}
