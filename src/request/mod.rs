//! Types related to OAuth 2.0 requests.
//!
//! - [`AuthorizationRequest`] — request to the authorization endpoint (RFC 6749 Section 4.1.1 / 4.2.1)
//! - [`TokenRequest`] — request to the token endpoint (RFC 6749 Section 4.x)
//! - [`TokenRequestError`] — token request validation error

mod authorization_request;
mod token_request;

pub use authorization_request::*;
pub use token_request::*;
