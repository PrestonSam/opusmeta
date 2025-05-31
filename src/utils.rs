#[derive(Debug, Clone)]
pub struct LowercaseString(pub String);

impl LowercaseString {
    pub fn new(str: &str) -> Self {
        Self(str.to_ascii_lowercase())
    }
}

impl<'a, T> From<T> for LowercaseString
where T: AsRef<str>
{
    fn from(value: T) -> Self {
        LowercaseString::new(value.as_ref())
    }
}
