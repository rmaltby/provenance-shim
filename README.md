Provenance Shim
A lightweight, language-aware provenance system for source files. The shim tracks the evolution of each file over time without polluting code or requiring specialized tooling.
Provenance is stored externally in .provenance/, while each file contains a single pointer comment referencing its chain.
What It Does
- Maintains a provenance chain for every file in your repository
- Stores provenance in human-readable YAML under .provenance/chains/
- Inserts a language-appropriate pointer comment at the top of each file
- Ensures idempotent annotation (safe to run repeatedly)
- Provides a CLI (prov) for annotation, stripping, and chain inspection
- Integrates cleanly with Git via a pre-commit hook
Directory Layout
.provenance/
chains/
<path>.yaml
Example:
src/lib.rs
.provenance/chains/src/lib.rs.yaml
Pointer Comments
Rust:
// provenance: .provenance/chains/src/lib.rs.yaml
Python:
provenance: .provenance/chains/foo.py.yaml
Markdown:
<!-- provenance: .provenance/chains/README.md.yaml -->
The shim automatically chooses the correct comment style.
CLI Usage
Annotate a file:
prov annotate --file src/lib.rs
This will:
- Strip any existing pointer comment
- Generate a new provenance entry
- Append it to .provenance/chains/src/lib.rs.yaml
- Insert a fresh pointer comment at the top of the file
Strip a pointer comment:
prov strip --file src/lib.rs
View a chain:
prov chain --file src/lib.rs
Git Integration
Add a pre-commit hook that runs prov annotate on staged files.
A template hook is provided in scripts/pre-commit.
Roadmap
- Diff hashing
- Provenance signing
- VS Code extension
- GitHub Action for provenance enforcement
- WASM bindings
- Configurable .provenance/config.yaml
Philosophy
The shim is designed to make provenance:
- Effortless
- Transparent
- Language-aware
- Human-readable
- Ecosystem-friendly
It aims to seed provenance as a cultural norm across open-source and AI-assisted development.
