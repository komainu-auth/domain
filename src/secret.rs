use std::fmt;
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

/// Generic wrapper type for holding secret data safely.
///
/// By centralizing `Debug`, `Display`, `PartialEq`, and `Eq` in this type,
/// individual value objects have no room to introduce leakage paths.
///
/// # Security Properties
///
/// - `Debug` and `Display` always output `"[REDACTED]"` and never leak the
///   internal value.
/// - `PartialEq` uses constant-time comparison via [`subtle::ConstantTimeEq`] to
///   prevent timing attacks.
/// - On `Drop`, calls [`zeroize::Zeroize::zeroize`] to zero out memory.
///
/// # Type Constraints
///
/// The type parameter `T` must satisfy `AsRef<[u8]> + Zeroize`. Common examples
/// are `String` and `Vec<u8>`.
///
/// # Examples
///
/// ```rust,ignore
/// use domain::secret::Secret;
///
/// let secret = Secret::new("my-password".to_string());
/// assert_eq!(format!("{secret}"), "[REDACTED]");
/// let raw = secret.expose_secret();
/// ```
pub struct Secret<T: AsRef<[u8]> + Zeroize>(T);

impl<T: AsRef<[u8]> + Zeroize> Secret<T> {
    /// Wraps secret data and creates a new [`Secret`].
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Returns a reference to the wrapped secret data.
    ///
    /// Using the explicit name `expose_secret` instead of `value()` encourages
    /// callers to recognize that they are deliberately extracting secret data.
    pub fn expose_secret(&self) -> &T {
        &self.0
    }
}

impl<T: AsRef<[u8]> + Zeroize + Clone> Clone for Secret<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: AsRef<[u8]> + Zeroize> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T: AsRef<[u8]> + Zeroize> fmt::Display for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T: AsRef<[u8]> + Zeroize> PartialEq for Secret<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().ct_eq(other.0.as_ref()).into()
    }
}

impl<T: AsRef<[u8]> + Zeroize> Eq for Secret<T> {}

impl<T: AsRef<[u8]> + Zeroize> Drop for Secret<T> {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn expose_secret_returns_original_value() {
        let secret = Secret::new("super-secret".to_string());
        assert_eq!(secret.expose_secret(), "super-secret");
    }

    #[test]
    fn debug_does_not_leak_value() {
        let secret = Secret::new("super-secret".to_string());
        let debugged = format!("{:?}", secret);
        assert_eq!(debugged, "[REDACTED]");
        assert!(!debugged.contains("super-secret"));
    }

    #[test]
    fn display_does_not_leak_value() {
        let secret = Secret::new("super-secret".to_string());
        let displayed = secret.to_string();
        assert_eq!(displayed, "[REDACTED]");
        assert!(!displayed.contains("super-secret"));
    }

    #[test]
    fn equal_values_are_equal() {
        let a = Secret::new("same-value".to_string());
        let b = Secret::new("same-value".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn different_values_are_not_equal() {
        let a = Secret::new("value-a".to_string());
        let b = Secret::new("value-b".to_string());
        assert_ne!(a, b);
    }

    #[test]
    fn different_length_values_are_not_equal() {
        // subtleのct_eqは長さが異なる場合パニックせずfalseを返すことを確認
        let a = Secret::new("short".to_string());
        let b = Secret::new("much-longer-value".to_string());
        assert_ne!(a, b);
    }

    #[test]
    fn clone_is_independent_of_original() {
        let secret = Secret::new("hello".to_string());
        let cloned = secret.clone();
        drop(secret); // 元をdropしてもcloneは別バッファなので影響を受けない
        assert_eq!(cloned.expose_secret(), "hello");
    }

    // zeroizeの呼び出しを記録するためのダミー型。
    // 実際のメモリ内容がゼロ化されたかをunsafeに検証するのは未定義動作の
    // リスクがあるため避け、代わりに「Dropの実装がzeroize()を呼んでいるか」
    // というロジックレベルの検証に留める。
    #[derive(Clone)]
    struct ZeroizeFlag(Rc<Cell<bool>>);

    impl AsRef<[u8]> for ZeroizeFlag {
        fn as_ref(&self) -> &[u8] {
            b"dummy"
        }
    }

    impl zeroize::Zeroize for ZeroizeFlag {
        fn zeroize(&mut self) {
            self.0.set(true);
        }
    }

    #[test]
    fn drop_calls_zeroize() {
        let flag = Rc::new(Cell::new(false));
        let secret = Secret::new(ZeroizeFlag(flag.clone()));
        assert!(!flag.get());
        drop(secret);
        assert!(flag.get());
    }
}
