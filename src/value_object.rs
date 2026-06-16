use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait ValueObjectError: Error + Clone + PartialEq + Eq {}

pub trait ValueObject: Display + Debug + Clone + PartialEq + Eq + Hash {
    type Value;
    type Error: ValueObjectError;

    fn new(value: Self::Value) -> Result<Self, Self::Error>;
    fn value(&self) -> &Self::Value;
    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error>;
    fn is_valid(value: &Self::Value) -> Result<(), Self::Error>;
}

pub trait SecretValueObject: Display + Debug + Clone + PartialEq + Eq {
    type Value;
    type Error: ValueObjectError;

    fn new(value: Self::Value) -> Result<Self, Self::Error>;
    fn expose_secret(&self) -> &Self::Value; // value() ではなく明示的な名前
    fn set_value(&mut self, value: Self::Value) -> Result<(), Self::Error>;
    fn is_valid(value: &Self::Value) -> Result<(), Self::Error>;
}

pub trait ValueEnum: Display + Debug + Clone + PartialEq + Eq {}
