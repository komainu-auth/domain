use std::{error::Error, fmt::Debug};

/// Constraint trait that entity-related errors must satisfy.
///
/// Composes `Error + Clone + PartialEq + Eq` to guarantee that errors can be
/// treated as domain errors.
pub trait EntityError: Error + Clone + PartialEq + Eq {}

/// Foundation trait representing a domain entity.
///
/// An entity is an object identified by identity. Even if its values change,
/// it is considered the same entity as long as [`id`] remains the same.
///
/// # Type Parameters
///
/// - `Id` — The type of the identifier that uniquely identifies the entity.
///   Must satisfy `PartialEq + Eq + Clone`.
///
/// # Examples
///
/// ```rust,ignore
/// use domain::entity::Entity;
///
/// struct MyEntity { id: String, value: i32 }
///
/// impl Entity for MyEntity {
///     type Id = String;
///     fn id(&self) -> &Self::Id { &self.id }
/// }
/// ```
pub trait Entity: Debug + Clone {
    /// The type of the identifier that uniquely identifies the entity.
    type Id: PartialEq + Eq + Clone;

    /// Returns this entity's identifier.
    fn id(&self) -> &Self::Id;

    /// Returns whether this entity has the same identity as `other`.
    ///
    /// Entity identity is determined solely by the identifier. Returns `true`
    /// if the identifiers are equal, even when other field values differ.
    fn same_identity_as(&self, other: &Self) -> bool {
        self.id().eq(other.id())
    }
}
