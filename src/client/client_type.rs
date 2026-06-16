use std::fmt;

use crate::value_object::ValueEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientType {
    Public,
    Confidential
}

impl ValueEnum for ClientType {}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientType::Public => write!(f, "public"),
            ClientType::Confidential => write!(f, "confidential"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variants() {
        assert_eq!(ClientType::Public.to_string(), "public");
        assert_eq!(ClientType::Confidential.to_string(), "confidential");
    }

    #[test]
    fn equal_variants_are_equal() {
        assert_eq!(ClientType::Public, ClientType::Public);
        assert_eq!(ClientType::Confidential, ClientType::Confidential);
    }

    #[test]
    fn different_variants_are_not_equal() {
        assert_ne!(ClientType::Public, ClientType::Confidential);
    }

    #[test]
    fn clone_produces_equal_value() {
        let original = ClientType::Confidential;
        assert_eq!(original, original.clone());
    }
}
