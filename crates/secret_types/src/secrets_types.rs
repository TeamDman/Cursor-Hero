use std::fmt::Debug;
use zeroize::Zeroize;

use serde::Deserialize;
use serde::Serialize;

// shout out to the secrecy crate
// ran into some issues with reflect so had to roll my own tho
// https://github.com/iqlusioninc/crates/issues/632

use bevy::prelude::*;

#[derive(Reflect, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SecretString {
    inner: String,
}

impl SecretString {
    pub fn new(secret: String) -> Self {
        SecretString { inner: secret }
    }
    pub fn expose_secret(&self) -> &String {
        &self.inner
    }
}

impl Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.inner.len() {
                0 => "<empty>",
                _ => "<redacted>",
            }
        )
    }
}
impl Drop for SecretString {
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}
