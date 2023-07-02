use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(Arc<str>);

impl Name {
    pub fn new(name: Arc<str>) -> Name {
        Name(name)
    }

    pub fn duplicate(&self) -> Name {
        let str = &**self;
        Name::from(str)
    }
}

impl Default for Name {
    fn default() -> Self {
        Name(Arc::from(""))
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

impl std::ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        Name(Arc::clone(&self.0))
    }
}
