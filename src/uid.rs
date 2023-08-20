use std::{fmt, sync::Arc};

use rand::prelude::*;

// TODO: make sure it's not colliding

const SOUP: &str =
    "!#%()*+,-./:;=?@[]^_`{|}~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SOUP_LEN: usize = SOUP.len();
const UID_LEN: usize = 20;

fn generate() -> String {
    let mut rng = thread_rng();
    let uid = (0..UID_LEN)
        .map(|_| SOUP.as_bytes()[rng.gen_range(0..SOUP_LEN)])
        .collect();
    String::from_utf8(uid).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uid(Arc<str>);

impl Uid {
    pub fn generate() -> Uid {
        Uid::from(generate())
    }

    pub fn generate_string() -> String {
        generate()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// impl From<String> for Uid {
//     fn from(value: String) -> Self {
//         Uid(Arc::from(value))
//     }
// }

// impl From<&str> for Uid {
//     fn from(value: &str) -> Self {
//         Uid(Arc::from(value))
//     }
// }

impl<'a> From<&'a Uid> for &'a str {
    fn from(val: &'a Uid) -> Self {
        val.as_str()
    }
}

impl Default for Uid {
    fn default() -> Self {
        Uid::generate()
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
