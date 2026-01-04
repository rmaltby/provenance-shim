// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use sha2::{Sha256, Digest};

pub fn hash_prompt(prompt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prompt.as_bytes());
    let result = hasher.finalize();
    format!("sha256:{:x}", result)
}