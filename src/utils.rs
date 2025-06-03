use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct LowercaseString(pub(crate) String);

impl Deref for LowercaseString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LowercaseString {
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
