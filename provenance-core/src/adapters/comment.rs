use std::path::Path;

pub trait CommentAdapter {
    /// Return a single-line pointer comment for a given chain path.
    fn pointer_comment(&self, chain_path: &str) -> String;

    /// Detect and strip an existing pointer comment.
    /// Returns (stripped_text, found_pointer?)
    fn strip_pointer_comment(&self, text: &str) -> (String, bool);
}

/// Simple hash-style adapter `# provenance: <path>`
pub struct HashCommentAdapter;

impl CommentAdapter for HashCommentAdapter {
    fn pointer_comment(&self, chain_path: &str) -> String {
        format!(
            "# provenance: {}",
            chain_path.to_owned()
        )
    }

    fn strip_pointer_comment(&self, input: &str) -> (String, bool) {
        let mut lines = Vec::new();
        let mut stripped = false;

        for line in input.lines() {
            if line.trim_start().starts_with("# provenance: ") && !stripped {
                stripped = true;
                continue;
            }
            lines.push(line);
        }

        (lines.join("\n") + "\n", stripped)
    }
}

pub struct SlashCommentAdapter;

impl CommentAdapter for SlashCommentAdapter {
    fn pointer_comment(&self, chain_path: &str) -> String {
        format!(
            "// provenance: {}",
            chain_path.to_owned()
        )
    }

    fn strip_pointer_comment(&self, input: &str) -> (String, bool) {
        let mut lines = Vec::new();
        let mut stripped = false;

        for line in input.lines() {
            if line.trim_start().starts_with("// provenance: ") && !stripped {
                stripped = true;
                continue;
            }
            lines.push(line);
        }

        (lines.join("\n") + "\n", stripped)
    }
}

pub struct HtmlCommentAdapter;

impl CommentAdapter for HtmlCommentAdapter {
    fn pointer_comment(&self, chain_path: &str) -> String {
        format!(
            "<!-- provenance: {} -->",
            chain_path.to_owned()
        )
    }

    fn strip_pointer_comment(&self, input: &str) -> (String, bool) {
        let mut lines = Vec::new();
        let mut stripped = false;

        for line in input.lines() {
            if line.trim_start().starts_with("<!-- provenance: ") && line.trim_end().ends_with(" -->") && !stripped {
                stripped = true;
                continue;
            }
            lines.push(line);
        }

        (lines.join("\n") + "\n", stripped)
    }
}

/// Choose adapter base on file extension (for now: always hash-style)
pub fn adapter_for_path(path: &Path) -> Box<dyn CommentAdapter> {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str() {
        // Hash style for py, sh, toml, yaml, yml
        "py" | "sh" | "toml" | "yaml" | "yml" => Box::new(HashCommentAdapter),

        // Slash style for rs, js, ts, c, h, cpp, hpp, java, go
        "rs" | "js" | "ts" | "c" | "h" | "cpp" | "hpp" | "java" | "go" => Box::new(SlashCommentAdapter),

        // Html style for md, html, htm, xml
        "md" | "html" | "htm" | "xml" => Box::new(HtmlCommentAdapter),

        // Default fallback
        _ => Box::new(HashCommentAdapter),
    }
}
