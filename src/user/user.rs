use crate::{
    entity::Entity,
    user::{PasswordHash, UserId, UserName},
};

/// Entity representing the resource owner (end user).
///
/// Represents the party that authorizes access to protected resources as the
/// OAuth 2.0 resource owner.
///
/// # Entity Identity
///
/// [`Entity::id`] returns [`UserId`].
///
/// # Password Hash
///
/// `password_hash` is `Option` because users authenticated via an external
/// identity provider do not need one.
///
/// [`Entity::id`]: crate::entity::Entity::id
#[derive(Debug, Clone)]
pub struct User {
    user_id: UserId,
    user_name: UserName,
    password_hash: Option<PasswordHash>,
}

impl User {
    pub fn new(user_id: UserId, user_name: UserName, password_hash: Option<PasswordHash>) -> User {
        Self {
            user_id,
            user_name,
            password_hash,
        }
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn user_name(&self) -> &UserName {
        &self.user_name
    }
    pub fn password_hash(&self) -> Option<&PasswordHash> {
        self.password_hash.as_ref()
    }
    /// Changes the username.
    pub fn change_user_name(&mut self, new_name: UserName) {
        self.user_name = new_name
    }

    /// Updates the password hash.
    ///
    /// Used for both initial assignment (`None` → `Some`) and password rotation.
    pub fn rotate_password_hash(&mut self, new_hash: PasswordHash) {
        self.password_hash = Some(new_hash)
    }
}

impl Entity for User {
    type Id = UserId;

    fn id(&self) -> &Self::Id {
        &self.user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::{SecretValueObject, ValueObject};

    fn user_id() -> UserId {
        UserId::new("user-1".to_string()).unwrap()
    }
    fn user_name() -> UserName {
        UserName::new("alice".to_string()).unwrap()
    }
    fn password_hash() -> PasswordHash {
        PasswordHash::new("hash-1".to_string()).unwrap()
    }

    #[test]
    fn getters_return_constructor_values() {
        let user = User::new(user_id(), user_name(), Some(password_hash()));
        assert_eq!(user.user_id(), &user_id());
        assert_eq!(user.user_name(), &user_name());
        assert_eq!(user.password_hash().unwrap().expose_secret(), "hash-1");
    }

    #[test]
    fn password_hash_can_be_absent() {
        let user = User::new(user_id(), user_name(), None);
        assert!(user.password_hash().is_none());
    }

    #[test]
    fn change_user_name_updates_only_name() {
        let mut user = User::new(user_id(), user_name(), None);
        user.change_user_name(UserName::new("bob".to_string()).unwrap());
        assert_eq!(user.user_name().value(), "bob");
        assert_eq!(user.user_id(), &user_id());
    }

    #[test]
    fn rotate_password_hash_sets_hash_from_none() {
        let mut user = User::new(user_id(), user_name(), None);
        user.rotate_password_hash(password_hash());
        assert_eq!(user.password_hash().unwrap().expose_secret(), "hash-1");
    }

    #[test]
    fn rotate_password_hash_replaces_existing_hash() {
        let mut user = User::new(user_id(), user_name(), Some(password_hash()));
        user.rotate_password_hash(PasswordHash::new("hash-2".to_string()).unwrap());
        assert_eq!(user.password_hash().unwrap().expose_secret(), "hash-2");
    }

    #[test]
    fn id_is_the_user_id() {
        let user = User::new(user_id(), user_name(), None);
        assert_eq!(user.id(), &user_id());
    }

    #[test]
    fn same_identity_uses_user_id_only() {
        let a = User::new(user_id(), user_name(), None);
        let b = User::new(
            user_id(),
            UserName::new("different".to_string()).unwrap(),
            Some(password_hash()),
        );
        assert!(a.same_identity_as(&b));
    }

    #[test]
    fn different_user_ids_are_not_same_identity() {
        let a = User::new(user_id(), user_name(), None);
        let b = User::new(
            UserId::new("user-2".to_string()).unwrap(),
            user_name(),
            None,
        );
        assert!(!a.same_identity_as(&b));
    }

    #[test]
    fn debug_does_not_leak_password_hash() {
        let user = User::new(user_id(), user_name(), Some(password_hash()));
        assert!(!format!("{user:?}").contains("hash-1"));
    }
}
