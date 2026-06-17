use crate::value_object::{ValueObject, ValueObjectError};
use std::fmt;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectUriError {
    Empty,
    InvalidFormat(url::ParseError),
    UnsupportedScheme(String),
    FragmentNotAllowed,
}

impl ValueObjectError for RedirectUriError {}

impl fmt::Display for RedirectUriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedirectUriError::Empty => write!(f, "redirect_uri must not be empty"),
            RedirectUriError::InvalidFormat(e) => {
                write!(f, "redirect_uri is invalid format: {e}")
            }
            RedirectUriError::UnsupportedScheme(scheme) => {
                write!(
                    f,
                    "redirect_uri scheme '{scheme}' is not allowed (must be http or https)"
                )
            }
            RedirectUriError::FragmentNotAllowed => {
                write!(f, "redirect_uri must not contain a fragment component")
            }
        }
    }
}

impl std::error::Error for RedirectUriError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RedirectUriError::InvalidFormat(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RedirectUri(String);

impl fmt::Display for RedirectUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ValueObject for RedirectUri {
    type Value = String;
    type Error = RedirectUriError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        Self::is_valid(&value)?;
        let url = Url::parse(&value.trim()).map_err(RedirectUriError::InvalidFormat)?;
        Ok(Self(url.to_string()))
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }

    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error> {
        Self::is_valid(&value)?;
        let url = Url::parse(&value.trim()).map_err(RedirectUriError::InvalidFormat)?;
        self.0 = url.to_string();
        Ok(())
    }

    fn is_valid(value: &Self::Value) -> Result<(), Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(RedirectUriError::Empty);
        }
        let url = Url::parse(trimmed).map_err(RedirectUriError::InvalidFormat)?;

        // スキーマのバリデーションはアプリレイヤーの責務。
        // なぜなら、サーバーがhttpを許容するのか、あるいはカスタムスキーマをサポートするのかはアプリレイヤーで決定されるから。
        // if url.scheme() != "https" && url.scheme() != "http" {
        //     return Err(RedirectUriError::UnsupportedScheme(
        //         url.scheme().to_string(),
        //     ));
        // }
        if url.fragment().is_some() {
            return Err(RedirectUriError::FragmentNotAllowed);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_succeeds() {
        let uri = RedirectUri::new("https://example.com/callback".to_string()).unwrap();
        assert_eq!(uri.value(), "https://example.com/callback");
    }

    #[test]
    fn empty_string_fails() {
        assert_eq!(
            RedirectUri::new("".to_string()),
            Err(RedirectUriError::Empty)
        );
    }

    #[test]
    fn whitespace_only_fails() {
        assert_eq!(
            RedirectUri::new("   ".to_string()),
            Err(RedirectUriError::Empty)
        );
    }

    #[test]
    fn invalid_format_fails() {
        assert!(matches!(
            RedirectUri::new("not-a-url".to_string()),
            Err(RedirectUriError::InvalidFormat(_))
        ));
    }

    #[test]
    fn fragment_not_allowed_fails() {
        assert_eq!(
            RedirectUri::new("https://example.com/callback#fragment".to_string()),
            Err(RedirectUriError::FragmentNotAllowed)
        );
    }

    #[test]
    fn http_scheme_is_allowed() {
        let uri = RedirectUri::new("http://example.com/callback".to_string()).unwrap();
        assert_eq!(uri.value(), "http://example.com/callback");
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_on_construction() {
        let uri = RedirectUri::new(" https://example.com/callback ".to_string()).unwrap();
        assert_eq!(uri.value(), "https://example.com/callback");
    }

    #[test]
    fn set_value_trims_and_updates() {
        let mut uri = RedirectUri::new("https://example.com/a".to_string()).unwrap();
        uri.set_value("  http://example.com/b  ".to_string())
            .unwrap();
        assert_eq!(uri.value(), "http://example.com/b");
    }

    #[test]
    fn set_value_rejects_invalid_and_keeps_old_value() {
        let mut uri = RedirectUri::new("https://example.com/a".to_string()).unwrap();
        assert_eq!(
            uri.set_value("   ".to_string()),
            Err(RedirectUriError::Empty)
        );
        assert_eq!(uri.value(), "https://example.com/a");
    }

    #[test]
    fn error_message_is_descriptive() {
        let empty = RedirectUri::new("".to_string()).unwrap_err();
        assert_eq!(empty.to_string(), "redirect_uri must not be empty");

        let fragment = RedirectUri::new("https://example.com/callback#x".to_string()).unwrap_err();
        assert_eq!(
            fragment.to_string(),
            "redirect_uri must not contain a fragment component"
        );

        let invalid = RedirectUri::new("not-a-url".to_string()).unwrap_err();
        assert!(
            invalid
                .to_string()
                .starts_with("redirect_uri is invalid format:")
        );
    }

    #[test]
    fn equal_values_are_equal() {
        let a = RedirectUri::new("https://example.com/callback".to_string()).unwrap();
        let b = RedirectUri::new("https://example.com/callback".to_string()).unwrap();
        assert_eq!(a, b);
    }
}
