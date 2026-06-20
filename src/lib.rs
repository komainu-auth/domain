//! OAuth 2.0 domain model crate.
//!
//! This crate provides types, traits, and errors that form the domain layer of
//! the OAuth 2.0 authorization framework based on RFC 6749.
//!
//! # Modules
//!
//! - [`client`] — OAuth client entity and related value objects
//! - [`code`] — authorization codes and their issuance records
//! - [`entity`] — entity foundation trait
//! - [`error`] — OAuth 2.0 error codes and error types
//! - [`request`] — authorization requests and token requests
//! - [`secret`] — generic wrapper for holding secrets safely
//! - [`session`] — session entity and session ID value object
//! - [`token`] — access tokens, refresh tokens, and related records
//! - [`user`] — user entity and related value objects
//! - [`value_object`] — value object foundation traits
//!
//! Types re-exported from the crate root ([`GrantType`], [`RedirectUri`],
//! [`ResponseType`], [`Scope`], [`State`]) are the most frequently used and
//! can be imported directly from the top level of the crate.

pub mod client;
pub mod code;
pub mod entity;
pub mod error;
pub mod request;
pub mod secret;
pub mod session;
pub mod token;
pub mod user;
pub mod value_object;

mod grant_type;
mod redirect_uri;
mod response_type;
mod scope;
mod state;

pub use grant_type::*;
pub use redirect_uri::*;
pub use response_type::*;
pub use scope::*;
pub use state::*;
