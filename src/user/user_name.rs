use crate::value_object::{ValueObject, ValueObjectError};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserNameError {
    Empty,
}

impl ValueObjectError for UserNameError {}

impl std::fmt::Display for UserNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserNameError::Empty => write!(f, "user_name must not be empty"),
        }
    }
}

impl std::error::Error for UserNameError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName(String);

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for UserName {
    type Value = String;
    type Error = UserNameError;

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
            return Err(UserNameError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let user_name: UserName = UserName::new("user_name-123".to_string()).unwrap();
        assert_eq!(user_name.value(), "user_name-123");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(UserName::new("".to_string()), Err(UserNameError::Empty));
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(UserName::new("   ".to_string()), Err(UserNameError::Empty));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_on_construction() {
        // 仕様変更: 前後の空白は除去されて格納される
        let user_name = UserName::new(" abc ".to_string()).unwrap();
        assert_eq!(user_name.value(), "abc");
    }

    #[test]
    fn set_value_trims_and_updates() {
        let mut user_name = UserName::new("a".to_string()).unwrap();
        user_name.set_value("  b  ".to_string()).unwrap();
        assert_eq!(user_name.value(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut user_name = UserName::new("a".to_string()).unwrap();
        assert_eq!(user_name.set_value("   ".to_string()), Err(UserNameError::Empty));
        assert_eq!(user_name.value(), "a"); // 元の値が保持される
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = UserName::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "user_name must not be empty");
    }

    #[test]
    fn equal_values_are_equal() {
        let a = UserName::new("x".to_string()).unwrap();
        let b = UserName::new("x".to_string()).unwrap();
        assert_eq!(a, b);
    }
}
