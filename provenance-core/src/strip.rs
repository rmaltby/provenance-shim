// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use regex::Regex;

/// Remove provenance blocks from a text arfifact.
///
/// This can be improved over time: start simple.
pub fn strip_provenance(text: &str) -> String {
    // Very basic: remove trailing `----\nprovenance: ...`
    let re = Regex::new(r"(?s)\n?---\s*provenance:\s*.*").unwrap();
    re.replace(text, "").to_string()
}