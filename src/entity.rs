use std::{error::Error, fmt::Debug};

pub trait EntityError: Error + Clone + PartialEq + Eq {}

pub trait Entity: Debug + Clone {
    type Id: PartialEq + Eq + Clone;

    fn id(&self) -> &Self::Id;

    fn same_identity_as(&self, other: &Self) -> bool {
        self.id().eq(other.id())
    }
}
