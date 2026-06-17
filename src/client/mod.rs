//! Types related to OAuth 2.0 clients.
//!
//! This module contains the following types:
//!
//! - [`ClientId`] — client identifier (value object)
//! - [`ClientSecret`] — client secret (secret value object)
//! - [`ClientType`] — client type (public / confidential)
//! - [`OAuthClient`] — OAuth client entity
//! - [`ClientTokenTtl`] — per-client token TTL settings

mod client_id;
mod client_secret;
mod client_type;
mod oauth_client;
mod token_ttl;

pub use client_id::*;
pub use client_secret::*;
pub use client_type::*;
pub use oauth_client::*;
pub use token_ttl::*;
