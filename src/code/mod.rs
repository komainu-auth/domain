//! Types related to OAuth 2.0 authorization codes.
//!
//! - [`AuthorizationCode`] — authorization code value object (secret)
//! - [`AuthorizationCodeRecord`] — authorization code issuance record (entity)

mod authorization_code;
mod record;

pub use authorization_code::*;
pub use record::*;
