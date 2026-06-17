//! Types related to users.
//!
//! - [`User`] — user entity
//! - [`UserId`] — user identifier (value object)
//! - [`UserName`] — username (value object)
//! - [`PasswordHash`] — password hash (secret value object)

mod password_hash;
#[allow(clippy::module_inception)]
mod user;
mod user_id;
mod user_name;

pub use password_hash::*;
pub use user::*;
pub use user_id::*;
pub use user_name::*;
