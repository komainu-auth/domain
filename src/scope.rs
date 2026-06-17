use crate::value_object::{ValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`Scope`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeError {
    /// The scope is empty or whitespace only.
    Empty,
}

impl ValueObjectError for ScopeError {}

impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScopeError::Empty => write!(f, "scope must not be empty"),
        }
    }
}

impl std::error::Error for ScopeError {}

/// Value object representing an OAuth 2.0 scope (RFC 6749 Section 3.3).
///
/// Rejects empty or whitespace-only values at construction time. Leading and
/// trailing whitespace is trimmed automatically.
///
/// # Examples
///
/// ```rust,ignore
/// use domain::Scope;
/// use domain::value_object::ValueObject;
///
/// let scope = Scope::new("read".to_string()).unwrap();
/// assert_eq!(scope.value(), "read");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope(String);

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for Scope {
    type Value = String;
    type Error = ScopeError;

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
            return Err(ScopeError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let scope: Scope = Scope::new("scope-123".to_string()).unwrap();
        assert_eq!(scope.value(), "scope-123");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(Scope::new("".to_string()), Err(ScopeError::Empty));
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(Scope::new("   ".to_string()), Err(ScopeError::Empty));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_on_construction() {
        // 仕様変更: 前後の空白は除去されて格納される
        let scope = Scope::new(" abc ".to_string()).unwrap();
        assert_eq!(scope.value(), "abc");
    }

    #[test]
    fn set_value_trims_and_updates() {
        let mut scope = Scope::new("a".to_string()).unwrap();
        scope.set_value("  b  ".to_string()).unwrap();
        assert_eq!(scope.value(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut scope = Scope::new("a".to_string()).unwrap();
        assert_eq!(scope.set_value("   ".to_string()), Err(ScopeError::Empty));
        assert_eq!(scope.value(), "a"); // 元の値が保持される
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = Scope::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "scope must not be empty");
    }

    #[test]
    fn equal_values_are_equal() {
        let a = Scope::new("x".to_string()).unwrap();
        let b = Scope::new("x".to_string()).unwrap();
        assert_eq!(a, b);
    }
}
