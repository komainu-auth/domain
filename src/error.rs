use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthErrorCode {
    AccessDenied,
    InvalidClient,
    InvalidGrant,
    InvalidRequest,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
    UnauthorizedClient,
    UnsupportedGrantType,
    UnsupportedResponseType,
}

impl std::fmt::Display for OAuthErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthErrorCode::AccessDenied => write!(f, "access_denied"),
            OAuthErrorCode::InvalidClient => write!(f, "invalid_client"),
            OAuthErrorCode::InvalidGrant => write!(f, "invalid_grant"),
            OAuthErrorCode::InvalidRequest => write!(f, "invalid_request"),
            OAuthErrorCode::InvalidScope => write!(f, "invalid_scope"),
            OAuthErrorCode::ServerError => write!(f, "server_error"),
            OAuthErrorCode::TemporarilyUnavailable => write!(f, "temporarily_unavailable"),
            OAuthErrorCode::UnauthorizedClient => write!(f, "unauthorized_client"),
            OAuthErrorCode::UnsupportedGrantType => write!(f, "unsupported_grant_type"),
            OAuthErrorCode::UnsupportedResponseType => write!(f, "unsupported_response_type"),
        }
    }
}

impl std::error::Error for OAuthErrorCode {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuthError {
    error: OAuthErrorCode,
    error_description: Option<String>,
    error_uri: Option<String>,
}

impl OAuthError {
    pub fn new(
        error: OAuthErrorCode,
        error_description: Option<String>,
        error_uri: Option<String>,
    ) -> Self {
        Self {
            error,
            error_description,
            error_uri,
        }
    }

    pub fn error(&self) -> &OAuthErrorCode {
        &self.error
    }
    pub fn error_description(&self) -> Option<&String> {
        self.error_description.as_ref()
    }
    pub fn error_uri(&self) -> Option<&String> {
        self.error_uri.as_ref()
    }
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error_description {
            Some(description) => write!(f, "{}: {description}", self.error),
            None => write!(f, "{}", self.error),
        }
    }
}

impl std::error::Error for OAuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn error_code_display_matches_rfc_wire_format() {
        assert_eq!(OAuthErrorCode::AccessDenied.to_string(), "access_denied");
        assert_eq!(OAuthErrorCode::InvalidClient.to_string(), "invalid_client");
        assert_eq!(OAuthErrorCode::InvalidGrant.to_string(), "invalid_grant");
        assert_eq!(OAuthErrorCode::InvalidRequest.to_string(), "invalid_request");
        assert_eq!(OAuthErrorCode::InvalidScope.to_string(), "invalid_scope");
        assert_eq!(OAuthErrorCode::ServerError.to_string(), "server_error");
        assert_eq!(
            OAuthErrorCode::TemporarilyUnavailable.to_string(),
            "temporarily_unavailable"
        );
        assert_eq!(
            OAuthErrorCode::UnauthorizedClient.to_string(),
            "unauthorized_client"
        );
        assert_eq!(
            OAuthErrorCode::UnsupportedGrantType.to_string(),
            "unsupported_grant_type"
        );
        assert_eq!(
            OAuthErrorCode::UnsupportedResponseType.to_string(),
            "unsupported_response_type"
        );
    }

    #[test]
    fn error_code_equality_and_clone() {
        assert_eq!(OAuthErrorCode::InvalidGrant, OAuthErrorCode::InvalidGrant);
        assert_ne!(OAuthErrorCode::InvalidGrant, OAuthErrorCode::InvalidClient);
        let code = OAuthErrorCode::ServerError;
        assert_eq!(code, code.clone());
    }

    #[test]
    fn getters_return_constructor_values() {
        let error = OAuthError::new(
            OAuthErrorCode::InvalidRequest,
            Some("missing parameter".to_string()),
            Some("https://example.com/errors/invalid_request".to_string()),
        );
        assert_eq!(error.error(), &OAuthErrorCode::InvalidRequest);
        assert_eq!(
            error.error_description(),
            Some(&"missing parameter".to_string())
        );
        assert_eq!(
            error.error_uri(),
            Some(&"https://example.com/errors/invalid_request".to_string())
        );
    }

    #[test]
    fn optional_fields_can_be_absent() {
        let error = OAuthError::new(OAuthErrorCode::ServerError, None, None);
        assert!(error.error_description().is_none());
        assert!(error.error_uri().is_none());
    }

    #[test]
    fn display_includes_description_when_present() {
        let error = OAuthError::new(
            OAuthErrorCode::InvalidGrant,
            Some("code expired".to_string()),
            None,
        );
        assert_eq!(error.to_string(), "invalid_grant: code expired");
    }

    #[test]
    fn display_falls_back_to_code_when_description_absent() {
        let error = OAuthError::new(OAuthErrorCode::InvalidGrant, None, None);
        assert_eq!(error.to_string(), "invalid_grant");
    }

    #[test]
    fn source_returns_underlying_error_code() {
        let error = OAuthError::new(OAuthErrorCode::InvalidClient, None, None);
        let source = error.source().expect("source must be present");
        assert_eq!(source.to_string(), "invalid_client");
    }

    #[test]
    fn clone_and_equality() {
        let error = OAuthError::new(
            OAuthErrorCode::InvalidScope,
            Some("unknown scope".to_string()),
            None,
        );
        assert_eq!(error, error.clone());

        let different = OAuthError::new(OAuthErrorCode::InvalidScope, None, None);
        assert_ne!(error, different);
    }
}
