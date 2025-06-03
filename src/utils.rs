#[derive(Debug, Clone)]
pub struct LowercaseString(pub String);

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
