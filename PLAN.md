# Provenance Shim — Updated Architecture & Roadmap

The provenance shim has evolved from a simple text annotator into a general-purpose,
ecosystem-friendly provenance layer designed to track the lineage of files over time
without polluting source code or disrupting developer workflows.

This document outlines the updated architecture, design principles, and roadmap.

---

## 1. Core Principles

### 1.1 Non-intrusive provenance
Provenance should never interfere with code, formatting, or tooling.  
All long-form provenance is stored externally in `.provenance/`.

### 1.2 Human-readable, trustable artifacts
Provenance chains use plain-text YAML with real file paths, not hashed identifiers.
Users should be able to inspect provenance with no special tooling.

### 1.3 Language-aware integration
Each file contains a single, language-appropriate pointer comment referencing its
provenance chain. No block comments, no duplication, no embedded YAML.

### 1.4 Idempotent annotation
Running the annotator multiple times must produce stable results:
- Strip existing pointer comment
- Generate new provenance entry
- Append entry to chain file
- Insert fresh pointer comment

### 1.5 Extensible adapters
Language-specific comment styles live in `provenance-core/adapters/comment/`.

---

## 2. Directory Layout

```
.provenance/
    chains/
        <path>.yaml        # one chain file per repo file
    config.yaml            # optional future configuration
```

Examples:

```
src/lib.rs
.provenance/chains/src/lib.rs.yaml

README.md
.provenance/chains/README.md.yaml
```

---

## 3. Provenance Chain Format

Each chain file is a YAML list of provenance entries:

```yaml
- timestamp: 2026-01-05T04:28:27Z
  model:
    name: unknown
    provider: null
    version: null
  environment:
    os: windows
    editor: null
    tool_name: prov
    tool_version: 0.1.0
  prompt_hash: sha256:...
  prompt_excerpt: ""
  diff_hash: sha256:...
  tags: {}
```

Chains grow over time as files evolve.

---

## 4. Pointer Comment Format

Each file contains a single pointer comment at the top:

Rust:
```rust
// provenance: .provenance/chains/src/lib.rs.yaml
```

Python:
```python
# provenance: .provenance/chains/foo.py.yaml
```

Markdown:
```markdown
<!-- provenance: .provenance/chains/README.md.yaml -->
```

Pointer comments are:
- always single-line
- always at the top (after shebang if present)
- never duplicated

---

## 5. Annotation Pipeline

1. **Strip existing pointer comment**  
   Detect and remove the previous pointer comment.

2. **Generate new provenance entry**  
   Using `provenance-core` (model, environment, prompt hash, diff hash).

3. **Append entry to chain file**  
   Create chain file if missing.

4. **Insert fresh pointer comment**  
   Using the language-specific comment adapter.

5. **Write updated file**  
   Re-stage in Git if running inside a hook.

This pipeline is idempotent and safe for repeated runs.

---

## 6. CLI Roadmap

### 6.1 `prov annotate`
- Updates chain file
- Inserts pointer comment
- Writes updated file

### 6.2 `prov strip`
- Removes pointer comment only
- Does not modify chain file

### 6.3 `prov chain`
- Prints the chain for a given file

### 6.4 `prov diff-hash`
- Computes a stable hash of the staged diff (future)

---

## 7. Git Integration

### 7.1 Pre-commit hook
- Detect staged files
- Run `prov annotate` on each
- Re-stage modified files

### 7.2 GitHub Action (future)
- Validate presence of pointer comments
- Validate chain integrity
- Optionally enforce provenance on PRs

---

## 8. Future Work

- Signing provenance entries
- WASM bindings for browser-based tools
- VS Code extension for inline provenance inspection
- Configurable ignore/include patterns
- Provenance visualization tools
