use std::time::SystemTime;

use crate::{
    Scope,
    client::ClientId,
    entity::Entity,
    token::{AccessToken, RefreshToken},
    user::UserId,
};

#[derive(Debug, Clone)]
pub struct AccessTokenRecord {
    token: AccessToken,
    client_id: ClientId,
    user_id: Option<UserId>,
    scope: Scope,
    issued_at: SystemTime,
    expires_at: SystemTime,
}

impl AccessTokenRecord {
    pub fn new(
        token: AccessToken,
        client_id: ClientId,
        user_id: Option<UserId>,
        scope: Scope,
        issued_at: SystemTime,
        expires_at: SystemTime,
    ) -> Self {
        Self {
            token,
            client_id,
            user_id,
            scope,
            issued_at,
            expires_at,
        }
    }
    pub fn token(&self) -> &AccessToken {
        &self.token
    }
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }
    pub fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
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

impl Entity for AccessTokenRecord {
    type Id = AccessToken;

    fn id(&self) -> &Self::Id {
        &self.token
    }
}

#[derive(Debug, Clone)]
pub struct RefreshTokenRecord {
    token: RefreshToken,
    client_id: ClientId,
    user_id: Option<UserId>,
    scope: Scope,
    issued_at: SystemTime,
    expires_at: SystemTime,
}

impl RefreshTokenRecord {
    pub fn new(
        token: RefreshToken,
        client_id: ClientId,
        user_id: Option<UserId>,
        scope: Scope,
        issued_at: SystemTime,
        expires_at: SystemTime,
    ) -> Self {
        Self {
            token,
            client_id,
            user_id,
            scope,
            issued_at,
            expires_at,
        }
    }
    pub fn token(&self) -> &RefreshToken {
        &self.token
    }
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }
    pub fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
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

impl Entity for RefreshTokenRecord {
    type Id = RefreshToken;

    fn id(&self) -> &Self::Id {
        &self.token
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
    fn access_token() -> AccessToken {
        AccessToken::new("access".to_string()).unwrap()
    }
    fn refresh_token() -> RefreshToken {
        RefreshToken::new("refresh".to_string()).unwrap()
    }
    fn client_id() -> ClientId {
        ClientId::new("client-1".to_string()).unwrap()
    }
    fn user_id() -> UserId {
        UserId::new("user-1".to_string()).unwrap()
    }
    fn scope() -> Scope {
        Scope::new("read".to_string()).unwrap()
    }

    #[test]
    fn access_token_record_getters_return_constructor_values() {
        let record = AccessTokenRecord::new(
            access_token(),
            client_id(),
            Some(user_id()),
            scope(),
            ts(100),
            ts(200),
        );
        assert_eq!(record.token().expose_secret(), "access");
        assert_eq!(record.client_id(), &client_id());
        assert_eq!(record.user_id(), Some(&user_id()));
        assert_eq!(record.scope(), &scope());
        assert_eq!(record.issued_at(), &ts(100));
        assert_eq!(record.expires_at(), &ts(200));
    }

    #[test]
    fn access_token_record_allows_absent_user_id() {
        let record =
            AccessTokenRecord::new(access_token(), client_id(), None, scope(), ts(100), ts(200));
        assert!(record.user_id().is_none());
    }

    #[test]
    fn access_token_record_id_is_the_token() {
        let record =
            AccessTokenRecord::new(access_token(), client_id(), None, scope(), ts(100), ts(200));
        assert_eq!(record.id(), &access_token());
    }

    #[test]
    fn access_token_record_same_identity_uses_token_only() {
        let a =
            AccessTokenRecord::new(access_token(), client_id(), None, scope(), ts(100), ts(200));
        let b = AccessTokenRecord::new(
            access_token(),
            ClientId::new("other-client".to_string()).unwrap(),
            Some(user_id()),
            Scope::new("write".to_string()).unwrap(),
            ts(1),
            ts(2),
        );
        assert!(a.same_identity_as(&b));
    }

    #[test]
    fn refresh_token_record_getters_return_constructor_values() {
        let record = RefreshTokenRecord::new(
            refresh_token(),
            client_id(),
            Some(user_id()),
            scope(),
            ts(100),
            ts(200),
        );
        assert_eq!(record.token().expose_secret(), "refresh");
        assert_eq!(record.client_id(), &client_id());
        assert_eq!(record.user_id(), Some(&user_id()));
        assert_eq!(record.scope(), &scope());
        assert_eq!(record.issued_at(), &ts(100));
        assert_eq!(record.expires_at(), &ts(200));
    }

    #[test]
    fn refresh_token_record_allows_absent_user_id() {
        let record = RefreshTokenRecord::new(
            refresh_token(),
            client_id(),
            None,
            scope(),
            ts(100),
            ts(200),
        );
        assert!(record.user_id().is_none());
    }

    #[test]
    fn refresh_token_record_id_is_the_token() {
        let record = RefreshTokenRecord::new(
            refresh_token(),
            client_id(),
            None,
            scope(),
            ts(100),
            ts(200),
        );
        assert_eq!(record.id(), &refresh_token());
    }

    #[test]
    fn debug_does_not_leak_token_secret() {
        let access = AccessTokenRecord::new(
            access_token(),
            client_id(),
            None,
            scope(),
            ts(100),
            ts(200),
        );
        let refresh = RefreshTokenRecord::new(
            refresh_token(),
            client_id(),
            None,
            scope(),
            ts(100),
            ts(200),
        );
        assert!(!format!("{access:?}").contains("access"));
        assert!(!format!("{refresh:?}").contains("refresh"));
    }
}
