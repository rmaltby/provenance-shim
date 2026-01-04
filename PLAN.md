# provenance-shim — Project Plan

This document outlines the current state of the project and the roadmap for
future development. The goal is to evolve `provenance-shim` into a small,
portable, widely‑adopted provenance layer for AI‑generated text.

---

# ✅ Current State (as of Jan 2026)

### Core Library (Rust)
- Provenance data model implemented
- Builder API implemented
- SHA‑256 prompt hashing implemented
- YAML + JSON serialization implemented
- Annotation (YAML front‑matter) implemented
- Provenance stripping implemented
- Example program working end‑to‑end
- Project builds cleanly on stable Rust

### Not yet implemented
- CLI wrapper
- Python bindings
- WASM build
- Editor integrations (VS Code, Neovim, JetBrains)
- Browser extension
- Git hooks
- Crates.io release
- Documentation site

---

# 🎯 Guiding Principles

1. **Minimalism**  
   The core should remain tiny, dependency‑light, and stable.

2. **Portability**  
   The Rust core should compile to native binaries and WASM, and be easy to wrap
   from Python, Node, and other environments.

3. **Interoperability**  
   Provenance blocks must be:
   - human‑readable  
   - machine‑parsable  
   - easy to strip  
   - easy to diff  

4. **Cultural adoption**  
   The long‑term goal is to make provenance a *norm*, not a burden.

---

# 🚀 Roadmap

## Phase 1 — Developer Tools (High Impact, Low Effort)
**Goal:** Make provenance easy to use in everyday workflows.

### 1. CLI Tool (`prov`)
- `prov annotate <file>`
- `prov strip <file>`
- `prov hash <string>`
- `prov version`
- Optional: `prov watch` for live annotation

### 2. Git Integration
- Pre‑commit hook template
- GitHub Action for provenance enforcement
- GitHub Action for provenance validation

### 3. Publish Rust crate
- Publish `provenance_core` to crates.io
- Add docs.rs documentation

---

## Phase 2 — Language Bindings (Broader Reach)

### 1. Python bindings
- `pip install provenance-shim`
- `provenance.annotate(text, prompt=...)`
- `provenance.strip(text)`
- `provenance.Provenance(...)`

### 2. Node/TypeScript bindings (optional)
- For integration with JS tooling and editors

### 3. WASM build
- Enables browser + VS Code + JetBrains integration

---

## Phase 3 — Editor Integrations (Cultural Norm Formation)

### 1. VS Code Extension
- Detect AI‑generated text insertions
- Auto‑append provenance blocks
- Command palette: “Insert Provenance Block”
- Status bar indicator

### 2. Neovim Plugin
- Lua wrapper around Rust core
- Commands: `:ProvAnnotate`, `:ProvStrip`

### 3. JetBrains Plugin
- WASM or JNI wrapper
- Auto‑annotation on paste or AI‑assist

---

## Phase 4 — Browser Extension

### Features:
- Detect AI‑generated text in textareas
- Add “Insert Provenance” button
- Optional auto‑annotation mode

Targets:
- GitHub
- StackOverflow
- Reddit
- Blogs
- Documentation editors

---

## Phase 5 — Community & Ecosystem

### 1. Documentation site
- Overview
- API docs
- Examples
- Integration guides

### 2. Blog posts / announcements
- “Why Provenance Matters”
- “Introducing provenance-shim”
- “A Minimal Standard for AI Provenance”

### 3. Templates
- README templates
- CONTRIBUTING.md templates
- Documentation templates

---

# 🧭 Long‑Term Vision

A world where:

- AI‑generated text carries its own lineage  
- Developers expect provenance the way they expect version control  
- Tools and editors treat provenance as a first‑class citizen  
- Training pipelines can distinguish human vs. synthetic data  
- The ecosystem avoids recursive contamination and epistemic drift  

`provenance-shim` is a small but foundational step toward that world.

---

# 📌 Status Summary

| Component            | Status |
|----------------------|--------|
| Rust core            | ✅ Done |
| CLI tool             | ⏳ Next |
| Python bindings      | ⏳ Planned |
| WASM build           | ⏳ Planned |
| VS Code extension    | ⏳ Planned |
| Browser extension    | ⏳ Planned |
| Git hooks            | ⏳ Planned |
| Crates.io release    | ⏳ Planned |

---

# 📝 Notes

This plan is intentionally modular. Each phase stands alone and can be
implemented independently or by different contributors. The Rust core is stable
enough to support all downstream integrations.
