use rand::prelude::*;

const SOUP: &str =
    "!#%()*+,-./:;=?@[]^_`{|}~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SOUP_LEN: usize = SOUP.len();
const UID_LEN: usize = 20;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uid(String);

pub fn uid() -> String {
    let mut rng = thread_rng();
    let uid = (0..UID_LEN)
        .map(|_| SOUP.as_bytes()[rng.gen_range(0..SOUP_LEN)])
        .collect();
    // SAFETY: We're taking from 'SOUP' const and they're all valid utf8 characters
    unsafe { String::from_utf8_unchecked(uid) }
}

impl Uid {
    pub fn generate() -> Uid {
        Uid(uid())
    }

    pub fn inner(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for Uid {
    fn from(value: String) -> Self {
        Uid(value)
    }
}

impl From<&str> for Uid {
    fn from(value: &str) -> Self {
        Uid(value.to_owned())
    }
}
