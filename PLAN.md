–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––– UPDATED PLAN.MD (PLAIN TEXT) ––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
Provenance Shim — Architecture & Roadmap
The provenance shim has evolved from a simple text annotator into a general-purpose, ecosystem-friendly provenance layer designed to track the lineage of files over time without polluting source code or disrupting developer workflows.
This document outlines the updated architecture, design principles, and roadmap.
1. 	Core Principles
1.1 Non-intrusive provenance
Provenance should never interfere with code, formatting, or tooling. All long-form provenance is stored externally in .provenance/.
1.2 Human-readable, trustable artifacts
Provenance chains use plain-text YAML with real file paths, not hashed identifiers. Users should be able to inspect provenance with no special tooling.
1.3 Language-aware integration
Each file contains a single, language-appropriate pointer comment referencing its provenance chain. No block comments, no duplication, no embedded YAML.
1.4 Idempotent annotation
Running the annotator multiple times must produce stable results:
• 	Strip existing pointer comment
• 	Generate new provenance entry
• 	Append entry to chain file
• 	Insert fresh pointer comment
1.5 Extensible adapters
Language-specific comment styles live in provenance-core/adapters/comment/.
2. 	Directory Layout
.provenance/
chains/
<path>.yaml
config.yaml (optional future)
Examples:
src/lib.rs
.provenance/chains/src/lib.rs.yaml
README.md
.provenance/chains/README.md.yaml
3. 	Provenance Chain Format
Each chain file is a YAML list of provenance entries. Each entry includes:
• 	timestamp
• 	generator
• 	model info
• 	environment info
• 	prompt hash
• 	prompt excerpt
• 	diff hash (future)
• 	sources
• 	tags
Chains grow over time as files evolve.
4. 	Pointer Comment Format
Each file contains a single pointer comment at the top.
Rust:
// provenance: .provenance/chains/src/lib.rs.yaml
Python:
provenance: .provenance/chains/foo.py.yaml
Markdown:
<!-- provenance: .provenance/chains/README.md.yaml -->
Pointer comments are always single-line, always at the top (after shebang if present), and never duplicated.
5. 	Annotation Pipeline
6. 	Strip existing pointer comment
7. 	Generate new provenance entry
8. 	Append entry to chain file
9. 	Insert fresh pointer comment
10. 	Write updated file
This pipeline is idempotent and safe for repeated runs.
6. 	CLI Roadmap
6.1 prov annotate
• 	Updates chain file
• 	Inserts pointer comment
• 	Writes updated file
6.2 prov strip
• 	Removes pointer comment only
• 	Does not modify chain file
6.3 prov chain
• 	Prints the chain for a given file
6.4 prov diff-hash (future)
• 	Computes a stable hash of the staged diff
7. 	Git Integration
7.1 Pre-commit hook
• 	Detect staged files
• 	Run prov annotate on each
• 	Re-stage modified files
7.2 GitHub Action (future)
• 	Validate presence of pointer comments
• 	Validate chain integrity
• 	Optionally enforce provenance on PRs
8. 	Future Work
• 	Signing provenance entries
• 	WASM bindings
• 	VS Code extension
• 	Configurable ignore/include patterns
• 	Provenance visualization tools
