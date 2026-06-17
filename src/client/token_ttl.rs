use std::time::Duration;

/// Per-client token time-to-live (TTL) settings.
///
/// `None` for each field means no client-specific setting; the server default
/// TTL is used. `Some(duration)` overrides the default for that client.
#[derive(Debug, Clone)]
pub struct ClientTokenTtl {
    access_token_ttl: Option<Duration>,
    refresh_token_ttl: Option<Duration>,
    authorization_code_ttl: Option<Duration>,
}

impl ClientTokenTtl {
    /// Creates a new [`ClientTokenTtl`].
    ///
    /// Passing `None` for any argument means no client-specific setting (use
    /// the server default).
    pub fn new(
        access_token_ttl: Option<Duration>,
        refresh_token_ttl: Option<Duration>,
        authorization_code_ttl: Option<Duration>,
    ) -> Self {
        Self {
            access_token_ttl,
            refresh_token_ttl,
            authorization_code_ttl,
        }
    }

    /// Returns the access token TTL. `None` means the server default is used.
    pub fn access_token_ttl(&self) -> Option<&Duration> {
        self.access_token_ttl.as_ref()
    }

    /// Returns the refresh token TTL. `None` means the server default is used.
    pub fn refresh_token_ttl(&self) -> Option<&Duration> {
        self.refresh_token_ttl.as_ref()
    }

    /// Returns the authorization code TTL. `None` means the server default is used.
    pub fn authorization_code_ttl(&self) -> Option<&Duration> {
        self.authorization_code_ttl.as_ref()
    }

    /// Sets the access token TTL. Passing `None` reverts to the server default.
    pub fn set_access_token_ttl(&mut self, access_token_ttl: Option<Duration>) {
        self.access_token_ttl = access_token_ttl
    }

    /// Sets the refresh token TTL. Passing `None` reverts to the server default.
    pub fn set_refresh_token_ttl(&mut self, refresh_token_ttl: Option<Duration>) {
        self.refresh_token_ttl = refresh_token_ttl
    }

    /// Sets the authorization code TTL. Passing `None` reverts to the server default.
    pub fn set_authorization_code_ttl(&mut self, authorization_code_ttl: Option<Duration>) {
        self.authorization_code_ttl = authorization_code_ttl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_all_values() {
        let ttl = ClientTokenTtl::new(
            Some(Duration::from_secs(3600)),
            Some(Duration::from_secs(86400)),
            Some(Duration::from_secs(600)),
        );
        assert_eq!(ttl.access_token_ttl(), Some(&Duration::from_secs(3600)));
        assert_eq!(ttl.refresh_token_ttl(), Some(&Duration::from_secs(86400)));
        assert_eq!(
            ttl.authorization_code_ttl(),
            Some(&Duration::from_secs(600))
        );
    }

    #[test]
    fn new_allows_all_none() {
        let ttl = ClientTokenTtl::new(None, None, None);
        assert!(ttl.access_token_ttl().is_none());
        assert!(ttl.refresh_token_ttl().is_none());
        assert!(ttl.authorization_code_ttl().is_none());
    }

    #[test]
    fn setters_update_only_their_own_field() {
        // 各 setter が対応するフィールドだけを更新する（コピペバグ検出）
        let mut ttl = ClientTokenTtl::new(None, None, None);

        ttl.set_access_token_ttl(Some(Duration::from_secs(1)));
        assert_eq!(ttl.access_token_ttl(), Some(&Duration::from_secs(1)));
        assert!(ttl.refresh_token_ttl().is_none());
        assert!(ttl.authorization_code_ttl().is_none());

        ttl.set_refresh_token_ttl(Some(Duration::from_secs(2)));
        assert_eq!(ttl.access_token_ttl(), Some(&Duration::from_secs(1)));
        assert_eq!(ttl.refresh_token_ttl(), Some(&Duration::from_secs(2)));
        assert!(ttl.authorization_code_ttl().is_none());

        ttl.set_authorization_code_ttl(Some(Duration::from_secs(3)));
        assert_eq!(ttl.access_token_ttl(), Some(&Duration::from_secs(1)));
        assert_eq!(ttl.refresh_token_ttl(), Some(&Duration::from_secs(2)));
        assert_eq!(ttl.authorization_code_ttl(), Some(&Duration::from_secs(3)));
    }

    #[test]
    fn setters_can_clear_to_none() {
        let mut ttl = ClientTokenTtl::new(
            Some(Duration::from_secs(1)),
            Some(Duration::from_secs(2)),
            Some(Duration::from_secs(3)),
        );
        ttl.set_access_token_ttl(None);
        ttl.set_refresh_token_ttl(None);
        ttl.set_authorization_code_ttl(None);
        assert!(ttl.access_token_ttl().is_none());
        assert!(ttl.refresh_token_ttl().is_none());
        assert!(ttl.authorization_code_ttl().is_none());
    }
}
