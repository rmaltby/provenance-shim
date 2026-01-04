// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use crate::model::Provenance;
use crate::serialize::to_yaml;

/// Append a provenance block to the end of a text artifact.
///
/// Format:
///
/// ```
/// <original text>
///
/// ---
/// provenance:
///    ... yaml ...
/// ```
///
pub fn annotate_text(text: &str, prov: &Provenance) -> Result<String, String> {
    let yaml = to_yaml(prov).map_err(|e| e.to_string())?;
    let mut out = String::new();
    out.push_str(text.trim_end());
    out.push_str("\n\n---\nprovenance:\n");

    // Indent YAML for nesting under 'provenance:'
    for line in yaml.lines() {
        out.push_str("   ");
        out.push_str(line);
        out.push('\n');
    }
    Ok(out)
}
