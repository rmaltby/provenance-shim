# provenance-shim
A Provenance Shim for AI coding agents and LLMs
# provenance-shim

A tiny, portable, language‑agnostic provenance layer for AI‑generated text.

`provenance-shim` provides a minimal Rust core library for attaching structured,
human‑readable, machine‑parsable provenance metadata to any text artifact. The
goal is to make provenance effortless, portable, and easy to integrate into
coding assistants, chatbots, editors, CLIs, and automation pipelines.

This project is intentionally small, dependency‑light, and designed to be
wrapped by many environments (Python, VS Code, browser extensions, Git hooks,
etc.). The long‑term vision is to help establish provenance as a cultural norm
for AI‑assisted content creation.

---

## License

Dual-licensed under **GPL-3.0-or-later OR Apache-2.0**.
You may choose either license.

## ✨ Features (current)

- **Rust core library** (`provenance_core`)
  - Provenance data model (generator, model info, environment, prompt hash, etc.)
  - Builder API for ergonomic construction
  - SHA‑256 prompt hashing
  - YAML + JSON serialization
  - Text annotation (YAML front‑matter style)
  - Provenance stripping (reversible)
- **Example program** (`cargo run --example main`)
  - Demonstrates end‑to‑end annotation

---

## 📦 Installation (Rust)

Add to your `Cargo.toml`:

```toml
[dependencies]
provenance_core = { path = "./provenance-core" }
```

Or once published:
```toml
[dependencies]
provenance_core = "0.x"
```

## 🧪 Example

```rust
use provenance_core::{
    Provenance, Generator, Environment, ModelInfo,
    hash_prompt, annotate_text,
};

fn main() {
    let prompt = "Explain the difference between a trait and a struct in Rust.";
    let output = "A struct defines data while a trait defines behavior.";

    let prov = Provenance::builder(Generator::AiAssistant)
        .environment(Environment::from_env())
        .model(ModelInfo {
            name: Some("unknown".into()),
            provider: None,
            version: None,
        })
        .prompt_hash(hash_prompt(prompt))
        .prompt_excerpt(prompt.chars().take(60).collect::<String>())
        .add_source("user_input")
        .build();

    let annotated = annotate_text(output, &prov).unwrap();
    println!("{annotated}");
}
```

Running:
```
cargo run --example main
```

Produces:
```
A struct defines data while a trait defines behavior.

---
provenance:
  generator: AiAssistant
  timestamp: 2026-01-04T06:06:39.768540Z
  model:
    name: unknown
    provider: null
    version: null
  environment:
    tool_name: null
    tool_version: null
    editor: null
    editor_version: null
    os: windows
  prompt_hash: sha256:...
  prompt_excerpt: Explain the difference between a trait and a struct in Rust.
  sources:
  - user_input
  tags: {}
```

🧹 Stripping provenance
```rust
let stripped = provenance_core::strip_provenance(&annotated);
```

🛠 Project Structure
```
provenance-shim/
  provenance-core/
    Cargo.toml
    src/
      annotate.rs
      builder.rs
      hash.rs
      lib.rs
      model.rs
      serialize.rs
      strip.rs
    examples/
      main.rs
```
