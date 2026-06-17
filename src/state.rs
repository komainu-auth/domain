use crate::value_object::{ValueObject, ValueObjectError};
use std::fmt;

/// Validation error for [`State`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    /// The state is empty or whitespace only.
    Empty,
}

impl ValueObjectError for StateError {}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::Empty => write!(f, "state must not be empty"),
        }
    }
}

impl std::error::Error for StateError {}

/// Value object representing the OAuth 2.0 state parameter (RFC 6749 Section 10.12).
///
/// Used to attach a random opaque value to authorization requests as a
/// cross-site request forgery (CSRF) countermeasure.
///
/// Rejects empty or whitespace-only values at construction time. Leading and
/// trailing whitespace is trimmed automatically.
///
/// # Examples
///
/// ```rust,ignore
/// use domain::State;
/// use domain::value_object::ValueObject;
///
/// let state = State::new("abc123xyz".to_string()).unwrap();
/// assert_eq!(state.value(), "abc123xyz");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State(String);

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for State {
    type Value = String;
    type Error = StateError;

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
            return Err(StateError::Empty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let state: State = State::new("state-123".to_string()).unwrap();
        assert_eq!(state.value(), "state-123");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(State::new("".to_string()), Err(StateError::Empty));
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(State::new("   ".to_string()), Err(StateError::Empty));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_on_construction() {
        // 仕様変更: 前後の空白は除去されて格納される
        let state = State::new(" abc ".to_string()).unwrap();
        assert_eq!(state.value(), "abc");
    }

    #[test]
    fn set_value_trims_and_updates() {
        let mut state = State::new("a".to_string()).unwrap();
        state.set_value("  b  ".to_string()).unwrap();
        assert_eq!(state.value(), "b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut state = State::new("a".to_string()).unwrap();
        assert_eq!(state.set_value("   ".to_string()), Err(StateError::Empty));
        assert_eq!(state.value(), "a"); // 元の値が保持される
    }

    #[test]
    fn error_message_is_descriptive() {
        let err = State::new("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "state must not be empty");
    }

    #[test]
    fn equal_values_are_equal() {
        let a = State::new("x".to_string()).unwrap();
        let b = State::new("x".to_string()).unwrap();
        assert_eq!(a, b);
    }
}
