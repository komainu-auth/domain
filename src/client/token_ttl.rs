use std::time::Duration;

/// クライアント単位のトークン有効期限（TTL）設定。
///
/// 各フィールドの `None` は「クライアント固有の設定なし」を表し、
/// サーバ既定の TTL を使用する意図である。`Some(duration)` の場合は
/// そのクライアントに対して既定値を上書きする。
#[derive(Debug, Clone)]
pub struct ClientTokenTtl {
    access_token_ttl: Option<Duration>,
    refresh_token_ttl: Option<Duration>,
    authorization_code_ttl: Option<Duration>,
}

impl ClientTokenTtl {
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

    pub fn access_token_ttl(&self) -> Option<&Duration> {
        self.access_token_ttl.as_ref()
    }
    pub fn refresh_token_ttl(&self) -> Option<&Duration> {
        self.refresh_token_ttl.as_ref()
    }
    pub fn authorization_code_ttl(&self) -> Option<&Duration> {
        self.authorization_code_ttl.as_ref()
    }

    pub fn set_access_token_ttl(&mut self, access_token_ttl: Option<Duration>) {
        self.access_token_ttl = access_token_ttl
    }
    pub fn set_refresh_token_ttl(&mut self, refresh_token_ttl: Option<Duration>) {
        self.refresh_token_ttl = refresh_token_ttl
    }
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
