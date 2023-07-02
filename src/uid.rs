use std::fmt;

use rand::prelude::*;

// TODO: make sure it's not colliding

const SOUP: &str =
    "!#%()*+,-./:;=?@[]^_`{|}~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SOUP_LEN: usize = SOUP.len();
const UID_LEN: usize = 20;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Uid(String);

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

fn generate() -> String {
    let mut rng = thread_rng();
    let uid = (0..UID_LEN)
        .map(|_| SOUP.as_bytes()[rng.gen_range(0..SOUP_LEN)])
        .collect();
    // SAFETY: We're taking from 'SOUP' const and they're all valid 1 byte utf8 characters
    unsafe { String::from_utf8_unchecked(uid) }
}

impl Uid {
    pub fn generate() -> Uid {
        Uid(generate())
    }

    pub fn from_string(string: String) -> Uid {
        Uid(string)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}
