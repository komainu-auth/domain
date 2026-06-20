use std::fmt;

use crate::{
    Scope, State,
    entity::Entity,
    user::UserId,
    value_object::{ValueObject, ValueObjectError},
};

/// Validation errors for [`SessionId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionIdError {
    /// The session ID is empty or contains only whitespace.
    Empty,
}

impl ValueObjectError for SessionIdError {}

impl std::fmt::Display for SessionIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionIdError::Empty => write!(f, "session_id must not be empty"),
        }
    }
}

impl std::error::Error for SessionIdError {}

/// Unique identifier for a [`SessionRecord`].
///
/// Wraps a non-empty string and trims surrounding whitespace on construction.
/// Implements [`ValueObject`] for validation and [`std::hash::Hash`] so it can
/// be used as a map key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for SessionId {
    type Value = String;
    type Error = SessionIdError;

    /// Creates a new [`SessionId`], trimming surrounding whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`SessionIdError::Empty`] if `value` is empty or contains only
    /// whitespace after trimming.
    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        Self::is_valid(&value)?;
        Ok(Self(value.trim().to_string()))
    }

    /// Returns a reference to the inner session ID string.
    fn value(&self) -> &Self::Value {
        &self.0
    }

    /// Replaces the inner value, trimming surrounding whitespace.
    ///
    /// The value is only updated when validation succeeds; on error the
    /// existing value is left unchanged.
    ///
    /// # Errors
    ///
    /// Returns [`SessionIdError::Empty`] if `value` is empty or contains only
    /// whitespace after trimming.
    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error> {
        Self::is_valid(&value)?;
        self.0 = value.trim().to_string();
        Ok(())
    }

    /// Validates a candidate session ID string.
    ///
    /// # Errors
    ///
    /// Returns [`SessionIdError::Empty`] if `value` is empty or contains only
    /// whitespace.
    fn is_valid(value: &Self::Value) -> Result<(), Self::Error> {
        if value.trim().is_empty() {
            return Err(SessionIdError::Empty);
        }
        Ok(())
    }
}

/// Entity that holds the state of a session.
///
/// Tracks login status and consent status independently. In OAuth 2.0 authorization
/// flows, user login and scope consent may occur in separate steps, so each flag is
/// managed on its own.
///
/// # Entity identity
///
/// [`Entity::id`] returns a [`SessionId`].
///
/// # Fields
///
/// - `session_id` — ID that uniquely identifies the session
/// - `user_id` — ID of the resource owner associated with the session
/// - `logged_in` — Whether the user is logged in for this session
/// - `consented` — Whether the user has completed consent to the requested scopes
///
/// [`Entity::id`]: komainu_domain::entity::Entity::id
#[derive(Debug, Clone)]
pub struct SessionRecord {
    session_id: SessionId,
    user_id: UserId,
    scopes: Vec<Scope>,
    state: Option<State>,
    logged_in: bool,
    consented: bool,
}

impl SessionRecord {
    /// Creates a new [`SessionRecord`].
    pub fn new(
        session_id: SessionId,
        user_id: UserId,
        scopes: Vec<Scope>,
        state: Option<State>,
        logged_in: bool,
        consented: bool,
    ) -> Self {
        Self {
            session_id,
            user_id,
            scopes,
            state,
            logged_in,
            consented,
        }
    }

    /// Returns the [`SessionId`] for this session.
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Returns the [`UserId`] of the user associated with this session.
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// Returns the slice of [`Scope`]s that the user was asked to consent to.
    pub fn scopes(&self) -> &[Scope] {
        self.scopes.as_ref()
    }

    /// Returns the optional [`State`] parameter carried by the authorization
    /// request, or `None` if the client did not supply one.
    pub fn state(&self) -> Option<&State> {
        self.state.as_ref()
    }

    /// Returns `true` if the user is logged in for this session.
    pub fn logged_in(&self) -> bool {
        self.logged_in
    }

    /// Returns `true` if the user has completed consent to the requested scopes.
    pub fn consented(&self) -> bool {
        self.consented
    }

    /// Records a successful login, setting `logged_in` to `true`.
    pub fn log_in_success(&mut self) {
        self.logged_in = true;
    }

    /// Records successful scope consent, setting `consented` to `true`.
    pub fn consent_success(&mut self) {
        self.consented = true;
    }
}

impl Entity for SessionRecord {
    type Id = SessionId;

    fn id(&self) -> &Self::Id {
        &self.session_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::ValueObject;

    // ---- SessionId ----

    #[test]
    fn session_id_valid_value_succeeds() {
        let id = SessionId::new("sess-abc".to_string()).unwrap();
        assert_eq!(id.value(), "sess-abc");
    }

    #[test]
    fn session_id_empty_string_fails() {
        assert_eq!(SessionId::new("".to_string()), Err(SessionIdError::Empty));
    }

    #[test]
    fn session_id_whitespace_only_fails() {
        assert_eq!(
            SessionId::new("   ".to_string()),
            Err(SessionIdError::Empty)
        );
    }

    #[test]
    fn session_id_surrounding_whitespace_is_trimmed() {
        let id = SessionId::new("  sess-1  ".to_string()).unwrap();
        assert_eq!(id.value(), "sess-1");
    }

    #[test]
    fn session_id_set_value_updates_on_success() {
        let mut id = SessionId::new("old".to_string()).unwrap();
        id.set_value("new".to_string()).unwrap();
        assert_eq!(id.value(), "new");
    }

    #[test]
    fn session_id_set_value_rejects_invalid_and_keeps_old() {
        let mut id = SessionId::new("old".to_string()).unwrap();
        assert_eq!(id.set_value("".to_string()), Err(SessionIdError::Empty));
        assert_eq!(id.value(), "old");
    }

    #[test]
    fn session_id_display() {
        let id = SessionId::new("sess-xyz".to_string()).unwrap();
        assert_eq!(id.to_string(), "sess-xyz");
    }

    #[test]
    fn session_id_equal_values_are_equal() {
        let a = SessionId::new("s".to_string()).unwrap();
        let b = SessionId::new("s".to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn session_id_error_display() {
        assert_eq!(
            SessionIdError::Empty.to_string(),
            "session_id must not be empty"
        );
    }

    #[test]
    fn session_id_error_implements_std_error() {
        let err: &dyn std::error::Error = &SessionIdError::Empty;
        assert_eq!(err.to_string(), "session_id must not be empty");
    }

    // ---- SessionRecord ----

    fn sample_session_id() -> SessionId {
        SessionId::new("sess-1".to_string()).unwrap()
    }
    fn sample_user_id() -> UserId {
        UserId::new("user-1".to_string()).unwrap()
    }
    fn sample_scopes() -> Vec<Scope> {
        vec![Scope::new("read".to_string()).unwrap()]
    }
    fn sample_state() -> Option<State> {
        Some(State::new("csrf-token".to_string()).unwrap())
    }
    fn sample_session_record(logged_in: bool, consented: bool) -> SessionRecord {
        SessionRecord::new(
            sample_session_id(),
            sample_user_id(),
            sample_scopes(),
            sample_state(),
            logged_in,
            consented,
        )
    }

    #[test]
    fn session_record_getters_return_constructor_values() {
        let record = sample_session_record(false, false);
        assert_eq!(record.session_id(), &sample_session_id());
        assert_eq!(record.user_id(), &sample_user_id());
        assert_eq!(record.scopes(), sample_scopes().as_slice());
        assert_eq!(record.state(), sample_state().as_ref());
        assert!(!record.logged_in());
        assert!(!record.consented());
    }

    #[test]
    fn log_in_success_sets_logged_in_to_true() {
        let mut record = sample_session_record(false, false);
        assert!(!record.logged_in());
        record.log_in_success();
        assert!(record.logged_in());
    }

    #[test]
    fn consent_success_sets_consented_to_true() {
        let mut record = sample_session_record(false, false);
        assert!(!record.consented());
        record.consent_success();
        assert!(record.consented());
    }

    #[test]
    fn log_in_success_does_not_affect_consented() {
        let mut record = sample_session_record(false, false);
        record.log_in_success();
        assert!(!record.consented());
    }

    #[test]
    fn consent_success_does_not_affect_logged_in() {
        let mut record = sample_session_record(false, false);
        record.consent_success();
        assert!(!record.logged_in());
    }

    #[test]
    fn session_record_id_is_session_id() {
        use crate::entity::Entity;
        let record = sample_session_record(false, false);
        assert_eq!(record.id(), &sample_session_id());
    }
}
