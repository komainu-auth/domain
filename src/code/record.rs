use std::time::SystemTime;

use crate::{
    RedirectUri, Scope, client::ClientId, code::AuthorizationCode, entity::Entity, user::UserId,
};

/// Entity holding authorization code issuance information.
///
/// Used to persist information when the authorization server issues an
/// authorization code in the authorization code grant flow (RFC 6749 Section 4.1).
///
/// # Entity Identity
///
/// [`Entity::id`] returns [`AuthorizationCode`], which serves as the unique
/// identifier.
///
/// # Fields
///
/// - `code` — authorization code (secret)
/// - `client_id` — identifier of the client that requested the code
/// - `user_id` — identifier of the authenticated resource owner
/// - `redirect_uri` — redirect URI associated with the code
/// - `scope` — granted scope
/// - `issued_at` — issuance timestamp
/// - `expires_at` — expiration timestamp
///
/// [`Entity::id`]: crate::entity::Entity::id
#[derive(Debug, Clone)]
pub struct AuthorizationCodeRecord {
    code: AuthorizationCode,
    client_id: ClientId,
    user_id: UserId,
    redirect_uri: RedirectUri,
    scope: Scope,
    issued_at: SystemTime,
    expires_at: SystemTime,
}

impl AuthorizationCodeRecord {
    pub fn new(
        code: AuthorizationCode,
        client_id: ClientId,
        user_id: UserId,
        redirect_uri: RedirectUri,
        scope: Scope,
        issued_at: SystemTime,
        expires_at: SystemTime,
    ) -> Self {
        Self {
            code,
            client_id,
            user_id,
            redirect_uri,
            scope,
            issued_at,
            expires_at,
        }
    }
    pub fn code(&self) -> &AuthorizationCode {
        &self.code
    }
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn redirect_uri(&self) -> &RedirectUri {
        &self.redirect_uri
    }
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    pub fn issued_at(&self) -> &SystemTime {
        &self.issued_at
    }
    pub fn expires_at(&self) -> &SystemTime {
        &self.expires_at
    }
}

impl Entity for AuthorizationCodeRecord {
    type Id = AuthorizationCode;

    fn id(&self) -> &Self::Id {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::{SecretValueObject, ValueObject};
    use std::time::Duration;

    fn ts(secs: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(secs)
    }
    fn code() -> AuthorizationCode {
        AuthorizationCode::new("auth-code".to_string()).unwrap()
    }
    fn client_id() -> ClientId {
        ClientId::new("client-1".to_string()).unwrap()
    }
    fn user_id() -> UserId {
        UserId::new("user-1".to_string()).unwrap()
    }
    fn redirect_uri() -> RedirectUri {
        RedirectUri::new("https://example.com/cb".to_string()).unwrap()
    }
    fn scope() -> Scope {
        Scope::new("read".to_string()).unwrap()
    }

    fn sample_record() -> AuthorizationCodeRecord {
        AuthorizationCodeRecord::new(
            code(),
            client_id(),
            user_id(),
            redirect_uri(),
            scope(),
            ts(100),
            ts(200),
        )
    }

    #[test]
    fn getters_return_constructor_values() {
        let record = sample_record();
        assert_eq!(record.code().expose_secret(), "auth-code");
        assert_eq!(record.client_id(), &client_id());
        assert_eq!(record.user_id(), &user_id());
        assert_eq!(record.redirect_uri(), &redirect_uri());
        assert_eq!(record.scope(), &scope());
        assert_eq!(record.issued_at(), &ts(100));
        assert_eq!(record.expires_at(), &ts(200));
    }

    #[test]
    fn id_is_the_code() {
        let record = sample_record();
        assert_eq!(record.id(), &code());
    }

    #[test]
    fn same_identity_uses_code_only() {
        let a = sample_record();
        let b = AuthorizationCodeRecord::new(
            code(),
            ClientId::new("other-client".to_string()).unwrap(),
            UserId::new("other-user".to_string()).unwrap(),
            redirect_uri(),
            Scope::new("write".to_string()).unwrap(),
            ts(1),
            ts(2),
        );
        assert!(a.same_identity_as(&b));
    }

    #[test]
    fn debug_does_not_leak_code_secret() {
        let record = sample_record();
        assert!(!format!("{record:?}").contains("auth-code"));
    }
}
