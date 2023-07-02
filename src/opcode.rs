use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpCode(Cow<'static, str>);

impl OpCode {
    pub fn new<S>(s: S) -> OpCode
    where
        S: Into<Cow<'static, str>>,
    {
        OpCode(s.into())
    }
}

impl From<String> for OpCode {
    fn from(value: String) -> Self {
        OpCode::new(value)
    }
}

impl From<&'static str> for OpCode {
    fn from(value: &'static str) -> Self {
        OpCode::new(value)
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
