use crate::{
    RedirectUri, Scope, client::ClientId, code::AuthorizationCode, grant_type::GrantType,
    token::RefreshToken, user::UserName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenRequestError {
    LackOfCode,
    LackOfUserName,
    LackOfPassword,
    LackOfRefreshToken,
    InvalidGrantType(GrantType),
}

impl std::fmt::Display for TokenRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LackOfCode => {
                write!(f, "authorization code grant request must have `code`")
            }
            Self::LackOfUserName => {
                write!(f, "password grant request must have `username`")
            }
            Self::LackOfPassword => {
                write!(f, "password grant request must have `password`")
            }
            Self::LackOfRefreshToken => {
                write!(f, "refresh token grant request must have `refresh_token`")
            }
            Self::InvalidGrantType(grant_type) => {
                write!(f, "grant_type must be {grant_type}")
            }
        }
    }
}

impl std::error::Error for TokenRequestError {}

#[derive(Debug, Clone)]
pub struct TokenRequest {
    grant_type: GrantType,
    code: Option<AuthorizationCode>,
    redirect_uri: Option<RedirectUri>,
    client_id: Option<ClientId>,
    username: Option<UserName>,
    password: Option<String>,
    refresh_token: Option<RefreshToken>,
    scope: Option<Scope>,
}

impl TokenRequest {
    pub fn new_authorization_code(
        code: AuthorizationCode,
        redirect_uri: Option<RedirectUri>,
        client_id: Option<ClientId>,
    ) -> Self {
        Self {
            grant_type: GrantType::AuthorizationCode,
            code: Some(code),
            redirect_uri,
            client_id,
            username: None,
            password: None,
            refresh_token: None,
            scope: None,
        }
    }

    pub fn new_password(username: UserName, password: String, scope: Option<Scope>) -> Self {
        Self {
            grant_type: GrantType::Password,
            code: None,
            redirect_uri: None,
            client_id: None,
            username: Some(username),
            password: Some(password),
            refresh_token: None,
            scope,
        }
    }

    pub fn new_client_credentials(scope: Option<Scope>) -> Self {
        Self {
            grant_type: GrantType::ClientCredentials,
            code: None,
            redirect_uri: None,
            client_id: None,
            username: None,
            password: None,
            refresh_token: None,
            scope,
        }
    }

    pub fn new_refresh_token(refresh_token: RefreshToken, scope: Option<Scope>) -> Self {
        Self {
            grant_type: GrantType::RefreshToken,
            code: None,
            redirect_uri: None,
            client_id: None,
            username: None,
            password: None,
            refresh_token: Some(refresh_token),
            scope,
        }
    }
}

impl TokenRequest {
    pub fn grant_type(&self) -> &GrantType {
        &self.grant_type
    }

    pub fn is_authorization_code_grant(&self) -> bool {
        matches!(&self.grant_type, GrantType::AuthorizationCode)
    }
    pub fn is_password_grant(&self) -> bool {
        matches!(&self.grant_type, GrantType::Password)
    }
    pub fn is_client_credentials_grant(&self) -> bool {
        matches!(&self.grant_type, GrantType::ClientCredentials)
    }
    pub fn is_refresh_token_grant(&self) -> bool {
        matches!(&self.grant_type, GrantType::RefreshToken)
    }
}

impl TokenRequest {
    pub fn code(&self) -> Result<&AuthorizationCode, TokenRequestError> {
        if !self.is_authorization_code_grant() {
            return Err(TokenRequestError::InvalidGrantType(
                GrantType::AuthorizationCode,
            ));
        }

        self.code.as_ref().ok_or(TokenRequestError::LackOfCode)
    }
    pub fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.redirect_uri.as_ref()
    }
    pub fn client_id(&self) -> Option<&ClientId> {
        self.client_id.as_ref()
    }

    pub fn username(&self) -> Result<&UserName, TokenRequestError> {
        if !self.is_password_grant() {
            return Err(TokenRequestError::InvalidGrantType(GrantType::Password));
        }

        self.username
            .as_ref()
            .ok_or(TokenRequestError::LackOfUserName)
    }
    pub fn password(&self) -> Result<&String, TokenRequestError> {
        if !self.is_password_grant() {
            return Err(TokenRequestError::InvalidGrantType(GrantType::Password));
        }

        self.password
            .as_ref()
            .ok_or(TokenRequestError::LackOfPassword)
    }

    pub fn refresh_token(&self) -> Result<&RefreshToken, TokenRequestError> {
        if !self.is_refresh_token_grant() {
            return Err(TokenRequestError::InvalidGrantType(GrantType::RefreshToken));
        }

        self.refresh_token
            .as_ref()
            .ok_or(TokenRequestError::LackOfRefreshToken)
    }

    pub fn scope(&self) -> Option<&Scope> {
        self.scope.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::{SecretValueObject, ValueObject};

    fn sample_code() -> AuthorizationCode {
        AuthorizationCode::new("auth-code".to_string()).unwrap()
    }
    fn sample_user_name() -> UserName {
        UserName::new("alice".to_string()).unwrap()
    }
    fn sample_refresh_token() -> RefreshToken {
        RefreshToken::new("refresh-token".to_string()).unwrap()
    }
    fn sample_scope() -> Scope {
        Scope::new("read".to_string()).unwrap()
    }

    #[test]
    fn new_authorization_code_sets_grant_type_and_predicates() {
        let req = TokenRequest::new_authorization_code(sample_code(), None, None);
        assert_eq!(req.grant_type(), &GrantType::AuthorizationCode);
        assert!(req.is_authorization_code_grant());
        assert!(!req.is_password_grant());
        assert!(!req.is_client_credentials_grant());
        assert!(!req.is_refresh_token_grant());
    }

    #[test]
    fn new_password_sets_grant_type_and_predicates() {
        let req = TokenRequest::new_password(sample_user_name(), "pw".to_string(), None);
        assert_eq!(req.grant_type(), &GrantType::Password);
        assert!(req.is_password_grant());
        assert!(!req.is_authorization_code_grant());
        assert!(!req.is_client_credentials_grant());
        assert!(!req.is_refresh_token_grant());
    }

    #[test]
    fn new_client_credentials_sets_grant_type_and_predicates() {
        let req = TokenRequest::new_client_credentials(None);
        assert_eq!(req.grant_type(), &GrantType::ClientCredentials);
        assert!(req.is_client_credentials_grant());
        assert!(!req.is_authorization_code_grant());
        assert!(!req.is_password_grant());
        assert!(!req.is_refresh_token_grant());
    }

    #[test]
    fn new_refresh_token_sets_grant_type_and_predicates() {
        let req = TokenRequest::new_refresh_token(sample_refresh_token(), None);
        assert_eq!(req.grant_type(), &GrantType::RefreshToken);
        assert!(req.is_refresh_token_grant());
        assert!(!req.is_authorization_code_grant());
        assert!(!req.is_password_grant());
        assert!(!req.is_client_credentials_grant());
    }

    #[test]
    fn authorization_code_grant_exposes_code() {
        let req = TokenRequest::new_authorization_code(sample_code(), None, None);
        assert_eq!(req.code().unwrap().expose_secret(), "auth-code");
    }

    #[test]
    fn password_grant_exposes_username_and_password() {
        let req = TokenRequest::new_password(sample_user_name(), "pw".to_string(), None);
        assert_eq!(req.username().unwrap().value(), "alice");
        assert_eq!(req.password().unwrap(), "pw");
    }

    #[test]
    fn refresh_token_grant_exposes_refresh_token() {
        let req = TokenRequest::new_refresh_token(sample_refresh_token(), None);
        assert_eq!(req.refresh_token().unwrap().expose_secret(), "refresh-token");
    }

    #[test]
    fn code_returns_invalid_grant_type_for_non_authorization_code_grant() {
        let req = TokenRequest::new_password(sample_user_name(), "pw".to_string(), None);
        assert_eq!(
            req.code().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::AuthorizationCode)
        );
    }

    #[test]
    fn username_returns_invalid_grant_type_for_non_password_grant() {
        let req = TokenRequest::new_authorization_code(sample_code(), None, None);
        assert_eq!(
            req.username().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::Password)
        );
    }

    #[test]
    fn password_returns_invalid_grant_type_for_non_password_grant() {
        let req = TokenRequest::new_authorization_code(sample_code(), None, None);
        assert_eq!(
            req.password().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::Password)
        );
    }

    #[test]
    fn refresh_token_returns_invalid_grant_type_for_non_refresh_token_grant() {
        let req = TokenRequest::new_authorization_code(sample_code(), None, None);
        assert_eq!(
            req.refresh_token().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::RefreshToken)
        );
    }

    #[test]
    fn client_credentials_grant_rejects_all_grant_specific_getters() {
        let req = TokenRequest::new_client_credentials(None);
        assert_eq!(
            req.code().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::AuthorizationCode)
        );
        assert_eq!(
            req.username().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::Password)
        );
        assert_eq!(
            req.password().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::Password)
        );
        assert_eq!(
            req.refresh_token().unwrap_err(),
            TokenRequestError::InvalidGrantType(GrantType::RefreshToken)
        );
    }

    #[test]
    fn optional_fields_default_to_none() {
        let req = TokenRequest::new_client_credentials(None);
        assert!(req.redirect_uri().is_none());
        assert!(req.client_id().is_none());
        assert!(req.scope().is_none());
    }

    #[test]
    fn scope_is_propagated() {
        let req = TokenRequest::new_password(
            sample_user_name(),
            "pw".to_string(),
            Some(sample_scope()),
        );
        assert_eq!(req.scope(), Some(&sample_scope()));
    }

    #[test]
    fn error_messages_are_descriptive() {
        assert_eq!(
            TokenRequestError::LackOfCode.to_string(),
            "authorization code grant request must have `code`"
        );
        assert_eq!(
            TokenRequestError::LackOfUserName.to_string(),
            "password grant request must have `username`"
        );
        assert_eq!(
            TokenRequestError::LackOfPassword.to_string(),
            "password grant request must have `password`"
        );
        assert_eq!(
            TokenRequestError::LackOfRefreshToken.to_string(),
            "refresh token grant request must have `refresh_token`"
        );
        assert_eq!(
            TokenRequestError::InvalidGrantType(GrantType::Password).to_string(),
            "grant_type must be password"
        );
    }
}
