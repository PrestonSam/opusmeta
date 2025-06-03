use std::ops::Deref;

/// A lowercase String. Holds a [`String`] internally.
#[derive(Debug, Clone)]
pub struct LowercaseString(pub(crate) String);

impl Deref for LowercaseString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LowercaseString {
    /// Create a new `LowercaseString`. This will copy the contents of the argument to a newly
    /// allocated buffer.
    #[must_use]
    pub fn new(str: &str) -> Self {
        Self(str.to_ascii_lowercase())
    }
}

impl<T> From<T> for LowercaseString
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::new(value.as_ref())
    }
}
