use std::time::Duration;

use crate::{
    Scope,
    token::{AccessToken, RefreshToken, TokenType},
};

#[derive(Debug, Clone)]
pub struct IssuedToken {
    access_token: AccessToken,
    token_type: TokenType,
    expires_in: Duration,
    refresh_token: Option<RefreshToken>,
    scope: Option<Scope>,
}

impl IssuedToken {
    pub fn new(
        access_token: AccessToken,
        token_type: TokenType,
        expires_in: Duration,
        refresh_token: Option<RefreshToken>,
        scope: Option<Scope>,
    ) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
            refresh_token,
            scope,
        }
    }
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn expires_in(&self) -> &Duration {
        &self.expires_in
    }
    pub fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }
    pub fn scope(&self) -> Option<&Scope> {
        self.scope.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::{SecretValueObject, ValueObject};

    fn access_token() -> AccessToken {
        AccessToken::new("at-secret-value".to_string()).unwrap()
    }
    fn refresh_token() -> RefreshToken {
        RefreshToken::new("rt-secret-value".to_string()).unwrap()
    }
    fn scope() -> Scope {
        Scope::new("read".to_string()).unwrap()
    }

    #[test]
    fn getters_return_constructor_values() {
        let token = IssuedToken::new(
            access_token(),
            TokenType::Bearer,
            Duration::from_secs(3600),
            Some(refresh_token()),
            Some(scope()),
        );
        assert_eq!(token.access_token().expose_secret(), "at-secret-value");
        assert_eq!(token.token_type(), &TokenType::Bearer);
        assert_eq!(token.expires_in(), &Duration::from_secs(3600));
        assert_eq!(
            token.refresh_token().unwrap().expose_secret(),
            "rt-secret-value"
        );
        assert_eq!(token.scope(), Some(&scope()));
    }

    #[test]
    fn optional_fields_can_be_absent() {
        let token = IssuedToken::new(
            access_token(),
            TokenType::Bearer,
            Duration::from_secs(60),
            None,
            None,
        );
        assert!(token.refresh_token().is_none());
        assert!(token.scope().is_none());
    }

    #[test]
    fn debug_does_not_leak_token_secrets() {
        let token = IssuedToken::new(
            access_token(),
            TokenType::Bearer,
            Duration::from_secs(60),
            Some(refresh_token()),
            None,
        );
        let debugged = format!("{token:?}");
        assert!(!debugged.contains("at-secret-value"));
        assert!(!debugged.contains("rt-secret-value"));
    }
}
