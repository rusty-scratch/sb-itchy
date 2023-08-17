use core::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(Arc<str>);

impl Name {
    pub fn new(name: Arc<str>) -> Name {
        Name(name)
    }

    /// Create a whole completely new arc.
    pub fn duplicate(&self) -> Name {
        let str = &**self;
        Name::from(str)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Name::new(Arc::from(value))
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name::new(Arc::from(value))
    }
}

impl<'a> From<&'a Name> for &'a str {
    fn from(value: &'a Name) -> Self {
        value.as_str()
    }
}

impl std::ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
