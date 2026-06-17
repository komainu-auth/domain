use std::{collections::HashSet, time::Duration};

use crate::{
    GrantType, RedirectUri, ResponseType, Scope,
    client::{ClientId, ClientSecret, ClientTokenTtl, ClientType},
    entity::{Entity, EntityError},
};

/// Consistency validation error for [`OAuthClient`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthClientError {
    /// Attempted to set a secret on a public client.
    PublicClientCannotHaveSecret,
    /// A confidential client has no secret set.
    ConfidentialClientMustHaveSecret,
}

impl std::fmt::Display for OAuthClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublicClientCannotHaveSecret => {
                write!(f, "public client cannot have a client_secret")
            }
            Self::ConfidentialClientMustHaveSecret => {
                write!(f, "confidential client must have a client_secret")
            }
        }
    }
}
impl std::error::Error for OAuthClientError {}
impl EntityError for OAuthClientError {}

/// OAuth 2.0 client entity.
///
/// Holds client information based on RFC 6749 Section 2. Validates the
/// presence or absence of a secret according to [`ClientType`].
///
/// # Consistency Rules
///
/// - [`ClientType::Confidential`] clients must always have a secret.
/// - [`ClientType::Public`] clients must not have a secret.
///
/// This validation runs in [`new`] and [`rotate_secret`].
///
/// # Entity Identity
///
/// [`Entity::id`] returns [`ClientId`].
///
/// [`new`]: OAuthClient::new
/// [`rotate_secret`]: OAuthClient::rotate_secret
/// [`Entity::id`]: crate::entity::Entity::id
#[derive(Debug, Clone)]
pub struct OAuthClient {
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    client_type: ClientType,
    redirect_uris: HashSet<RedirectUri>,
    grant_types: HashSet<GrantType>,
    response_types: HashSet<ResponseType>,
    scopes: HashSet<Scope>,
    token_ttl: ClientTokenTtl,
}

impl OAuthClient {
    /// Creates a new [`OAuthClient`].
    ///
    /// Validates consistency between `client_type` and `client_secret`.
    ///
    /// # Errors
    ///
    /// - When a confidential client has `None` for the secret:
    ///   [`OAuthClientError::ConfidentialClientMustHaveSecret`]
    /// - When a public client has `Some` for the secret:
    ///   [`OAuthClientError::PublicClientCannotHaveSecret`]
    pub fn new(
        client_id: ClientId,
        client_secret: Option<ClientSecret>,
        client_type: ClientType,
        token_ttl: ClientTokenTtl,
    ) -> Result<Self, OAuthClientError> {
        Self::is_consistent(&client_type, &client_secret)?;

        let redirect_uris = HashSet::new();
        let grant_types = HashSet::new();
        let response_types = HashSet::new();
        let scopes = HashSet::new();

        Ok(Self {
            client_id,
            client_secret,
            client_type,
            redirect_uris,
            grant_types,
            response_types,
            scopes,
            token_ttl,
        })
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }
    pub fn client_secret(&self) -> Option<&ClientSecret> {
        self.client_secret.as_ref()
    }
    pub fn client_type(&self) -> &ClientType {
        &self.client_type
    }
    pub fn redirect_uris(&self) -> &HashSet<RedirectUri> {
        &self.redirect_uris
    }
    pub fn grant_types(&self) -> &HashSet<GrantType> {
        &self.grant_types
    }
    pub fn response_types(&self) -> &HashSet<ResponseType> {
        &self.response_types
    }
    pub fn scopes(&self) -> &HashSet<Scope> {
        &self.scopes
    }
    pub fn access_token_ttl(&self) -> Option<&Duration> {
        self.token_ttl.access_token_ttl()
    }
    pub fn refresh_token_ttl(&self) -> Option<&Duration> {
        self.token_ttl.refresh_token_ttl()
    }
    pub fn authorization_code_ttl(&self) -> Option<&Duration> {
        self.token_ttl.authorization_code_ttl()
    }
    pub fn set_access_token_ttl(&mut self, access_token_ttl: Option<Duration>) {
        self.token_ttl.set_access_token_ttl(access_token_ttl)
    }
    pub fn set_refresh_token_ttl(&mut self, refresh_token_ttl: Option<Duration>) {
        self.token_ttl.set_refresh_token_ttl(refresh_token_ttl)
    }
    pub fn set_authorization_code_ttl(&mut self, authorization_code_ttl: Option<Duration>) {
        self.token_ttl
            .set_authorization_code_ttl(authorization_code_ttl)
    }
}

impl OAuthClient {
    /// Rotates the client secret.
    ///
    /// Only available for confidential clients.
    ///
    /// # Errors
    ///
    /// When called on a public client:
    /// [`OAuthClientError::PublicClientCannotHaveSecret`]
    pub fn rotate_secret(&mut self, new_secret: ClientSecret) -> Result<(), OAuthClientError> {
        match self.is_confidential() {
            true => {
                self.client_secret = Some(new_secret);
                Ok(())
            }
            false => Err(OAuthClientError::PublicClientCannotHaveSecret),
        }
    }
}

impl OAuthClient {
    pub fn add_redirect_uri(&mut self, uri: RedirectUri) {
        self.redirect_uris.insert(uri);
    }
    pub fn batch_add_redirect_uri(&mut self, uris: impl IntoIterator<Item = RedirectUri>) {
        for uri in uris {
            self.add_redirect_uri(uri);
        }
    }
    pub fn remove_redirect_uri(&mut self, uri: &RedirectUri) {
        self.redirect_uris.remove(uri);
    }
    pub fn batch_remove_redirect_uri(&mut self, uris: &[RedirectUri]) {
        for uri in uris {
            self.remove_redirect_uri(uri);
        }
    }
    pub fn supports_redirect_uri(&self, uri: &RedirectUri) -> bool {
        self.redirect_uris.contains(uri)
    }
}

impl OAuthClient {
    pub fn add_grant_type(&mut self, grant_type: GrantType) {
        self.grant_types.insert(grant_type);
    }
    pub fn batch_add_grant_type(&mut self, grant_types: impl IntoIterator<Item = GrantType>) {
        for grant_type in grant_types {
            self.add_grant_type(grant_type);
        }
    }
    pub fn remove_grant_type(&mut self, grant_type: &GrantType) {
        self.grant_types.remove(grant_type);
    }
    pub fn batch_remove_grant_type(&mut self, grant_types: &[GrantType]) {
        for grant_type in grant_types {
            self.remove_grant_type(grant_type);
        }
    }
    pub fn supports_grant_type(&self, grant_type: &GrantType) -> bool {
        self.grant_types.contains(grant_type)
    }
}

impl OAuthClient {
    pub fn add_response_type(&mut self, response_type: ResponseType) {
        self.response_types.insert(response_type);
    }
    pub fn batch_add_response_type(
        &mut self,
        response_types: impl IntoIterator<Item = ResponseType>,
    ) {
        for response_type in response_types {
            self.add_response_type(response_type);
        }
    }
    pub fn remove_response_type(&mut self, response_type: &ResponseType) {
        self.response_types.remove(response_type);
    }
    pub fn batch_remove_response_type(&mut self, response_types: &[ResponseType]) {
        for response_type in response_types {
            self.remove_response_type(response_type);
        }
    }
    pub fn supports_response_type(&self, response_type: &ResponseType) -> bool {
        self.response_types.contains(response_type)
    }
}

impl OAuthClient {
    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope);
    }
    pub fn batch_add_scope(&mut self, scopes: impl IntoIterator<Item = Scope>) {
        for scope in scopes {
            self.add_scope(scope);
        }
    }
    pub fn remove_scope(&mut self, scope: &Scope) {
        self.scopes.remove(scope);
    }
    pub fn batch_remove_scope(&mut self, scopes: &[Scope]) {
        for scope in scopes {
            self.remove_scope(scope);
        }
    }
    pub fn supports_scope(&self, scope: &Scope) -> bool {
        self.scopes.contains(scope)
    }
}

impl OAuthClient {
    /// Returns whether this is a confidential client.
    pub fn is_confidential(&self) -> bool {
        matches!(self.client_type, ClientType::Confidential)
    }

    /// Validates consistency between client type and secret.
    ///
    /// Used internally by [`new`] and [`rotate_secret`].
    ///
    /// # Errors
    ///
    /// Returns the corresponding [`OAuthClientError`] when consistency is violated.
    ///
    /// [`new`]: OAuthClient::new
    /// [`rotate_secret`]: OAuthClient::rotate_secret
    pub fn is_consistent(
        client_type: &ClientType,
        client_secret: &Option<ClientSecret>,
    ) -> Result<(), OAuthClientError> {
        match client_type {
            ClientType::Confidential => {
                if client_secret.is_none() {
                    return Err(OAuthClientError::ConfidentialClientMustHaveSecret);
                }
            }
            ClientType::Public => {
                if client_secret.is_some() {
                    return Err(OAuthClientError::PublicClientCannotHaveSecret);
                }
            }
        }
        Ok(())
    }
}

impl Entity for OAuthClient {
    type Id = ClientId;

    fn id(&self) -> &Self::Id {
        &self.client_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::{SecretValueObject, ValueObject};

    fn sample_client_id() -> ClientId {
        ClientId::new("client-123".to_string()).unwrap()
    }

    fn sample_secret() -> ClientSecret {
        ClientSecret::new("super-secret".to_string()).unwrap()
    }

    #[test]
    fn new_succeeds_for_confidential_with_secret() {
        let client = OAuthClient::new(
            sample_client_id(),
            Some(sample_secret()),
            ClientType::Confidential,
            ClientTokenTtl::new(None, None, None),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn new_fails_for_confidential_without_secret() {
        let result = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Confidential,
            ClientTokenTtl::new(None, None, None),
        );
        assert_eq!(
            result.unwrap_err(),
            OAuthClientError::ConfidentialClientMustHaveSecret
        );
    }

    #[test]
    fn new_succeeds_for_public_without_secret() {
        let client = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn new_fails_for_public_with_secret() {
        let result = OAuthClient::new(
            sample_client_id(),
            Some(sample_secret()),
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        );
        assert_eq!(
            result.unwrap_err(),
            OAuthClientError::PublicClientCannotHaveSecret
        );
    }

    #[test]
    fn new_creates_empty_collections() {
        let client = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();
        assert!(client.redirect_uris().is_empty());
        assert!(client.grant_types().is_empty());
        assert!(client.response_types().is_empty());
        assert!(client.scopes().is_empty());
    }

    #[test]
    fn rotate_secret_succeeds_for_confidential_client() {
        let mut client = OAuthClient::new(
            sample_client_id(),
            Some(sample_secret()),
            ClientType::Confidential,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();
        let new_secret = ClientSecret::new("new-secret".to_string()).unwrap();
        assert!(client.rotate_secret(new_secret).is_ok());
    }

    #[test]
    fn rotate_secret_fails_for_public_client() {
        let mut client = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();
        let new_secret = ClientSecret::new("new-secret".to_string()).unwrap();
        assert_eq!(
            client.rotate_secret(new_secret).unwrap_err(),
            OAuthClientError::PublicClientCannotHaveSecret
        );
    }

    #[test]
    fn redirect_uri_add_remove_and_supports() {
        let uri = RedirectUri::new("https://example.com/cb".to_string()).unwrap();
        let mut client = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();

        assert!(!client.supports_redirect_uri(&uri));
        client.add_redirect_uri(uri.clone());
        assert!(client.supports_redirect_uri(&uri));
        client.remove_redirect_uri(&uri);
        assert!(!client.supports_redirect_uri(&uri));
    }

    #[test]
    fn batch_add_and_remove_redirect_uris() {
        let uri_a = RedirectUri::new("https://example.com/a".to_string()).unwrap();
        let uri_b = RedirectUri::new("https://example.com/b".to_string()).unwrap();
        let mut client = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();

        client.batch_add_redirect_uri(vec![uri_a.clone(), uri_b.clone()]);
        assert_eq!(client.redirect_uris().len(), 2);

        client.batch_remove_redirect_uri(std::slice::from_ref(&uri_a));
        assert!(!client.supports_redirect_uri(&uri_a));
        assert!(client.supports_redirect_uri(&uri_b));
    }

    #[test]
    fn token_ttl_setters_update_correct_field() {
        // set_authorization_code_ttl のコピペバグを検出するテスト
        let mut client = OAuthClient::new(
            sample_client_id(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();

        client.set_access_token_ttl(Some(Duration::from_secs(3600)));
        client.set_refresh_token_ttl(Some(Duration::from_secs(86400)));
        client.set_authorization_code_ttl(Some(Duration::from_secs(600)));

        assert_eq!(client.access_token_ttl(), Some(&Duration::from_secs(3600)));
        assert_eq!(
            client.refresh_token_ttl(),
            Some(&Duration::from_secs(86400))
        );
        assert_eq!(
            client.authorization_code_ttl(),
            Some(&Duration::from_secs(600))
        );
    }

    #[test]
    fn debug_does_not_leak_secret_through_entity() {
        let client = OAuthClient::new(
            sample_client_id(),
            Some(sample_secret()),
            ClientType::Confidential,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();

        let debugged = format!("{:?}", client);
        assert!(!debugged.contains("super-secret"));
    }

    #[test]
    fn same_identity_as_ignores_other_fields() {
        let id = sample_client_id();
        let client_a = OAuthClient::new(
            id.clone(),
            None,
            ClientType::Public,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();
        let client_b = OAuthClient::new(
            id,
            Some(sample_secret()),
            ClientType::Confidential,
            ClientTokenTtl::new(None, None, None),
        )
        .unwrap();
        assert!(client_a.same_identity_as(&client_b));
    }
}
