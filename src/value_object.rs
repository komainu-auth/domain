use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

/// Constraint trait that value-object-related errors must satisfy.
///
/// Composes `Error + Clone + PartialEq + Eq` to guarantee that errors can be
/// treated as value object validation errors.
pub trait ValueObjectError: Error + Clone + PartialEq + Eq {}

/// Foundation trait representing a value object.
///
/// A value object is an object identified by the equality of its values.
/// Because it implements `Hash`, it can be used as a key in `HashSet` and
/// `HashMap`.
///
/// # Type Parameters
///
/// - `Value` — The type of the raw value held internally.
/// - `Error` — The error type returned on validation failure (implements
///   `ValueObjectError`).
///
/// # Contract
///
/// - [`new`] returns `Ok` only when `is_valid` succeeds.
/// - [`set_value`] does not modify the internal value on failure.
pub trait ValueObject: Display + Debug + Clone + PartialEq + Eq + Hash {
    /// The type of the raw value held.
    type Value;
    /// The validation error type.
    type Error: ValueObjectError;

    /// Validates the value and constructs an instance.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `is_valid` fails.
    fn new(value: Self::Value) -> Result<Self, Self::Error>;

    /// Returns a reference to the held value.
    fn value(&self) -> &Self::Value;

    /// Validates the value and then updates it.
    ///
    /// When validation fails, the internal value is not changed.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `is_valid` fails.
    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error>;

    /// Validates whether the given value is valid.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the value is invalid.
    fn is_valid(value: &Self::Value) -> Result<(), Self::Error>;
}

/// Foundation trait representing a value object that contains secret data.
///
/// Unlike [`ValueObject`], it uses [`expose_secret`] instead of `value()`.
/// This makes the intent to deliberately extract secret data explicit and
/// reduces the risk of accidental logging.
///
/// `Display` and `Debug` should output `[REDACTED]`.
pub trait SecretValueObject: Display + Debug + Clone + PartialEq + Eq {
    /// The type of the raw value held.
    type Value;
    /// The validation error type.
    type Error: ValueObjectError;

    /// Validates the value and constructs an instance.
    fn new(value: Self::Value) -> Result<Self, Self::Error>;

    /// Deliberately extracts the secret data.
    ///
    /// Using the explicit name `expose_secret` instead of `value()` forces
    /// callers to acknowledge that they are handling secret data.
    fn expose_secret(&self) -> &Self::Value;

    /// Validates the value and then updates it.
    ///
    /// When validation fails, the internal value is not changed.
    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error>;

    /// Validates whether the given value is valid.
    fn is_valid(value: &Self::Value) -> Result<(), Self::Error>;
}

/// Foundation trait representing an enum value object.
///
/// Composes `Display + Debug + Clone + PartialEq + Eq` and is implemented by
/// value types that can take only predefined choices (e.g. [`GrantType`],
/// [`ResponseType`]).
///
/// [`GrantType`]: crate::GrantType
pub trait ValueEnum: Display + Debug + Clone + PartialEq + Eq {}
