// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use serde::{Serialize, Deserialize};
use std:: collections::HashMap;
use chrono::{DateTime, Utc};

// Who/what produced this context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Generator {
    Human,
    AiAssistant,
    Hybrid,
    Other(String),
}

// Optional info about the AI model, if known
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: Option<String>,    // e.g. "gpt-4.5", "local-llama"
    pub provider: Option<String>, // e.g. "OpenAI", "Local", "Anthropic"
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub tool_name: Option<String>, // e.g. "vscode-provenance-shim"
    pub tool_version: Option<String>,
    pub editor: Option<String>, // e.g. "VSCode", "Neovim", "JetBrains"
    pub editor_version: Option<String>,
    pub os: Option<String>, // e.g. "windows", "linux", "macos"
}

impl Environment {
    pub fn new() -> Self {
        // Minimal, best-effort defaults.
        Environment {
            tool_name: std::env::var("PROV_TOOL_NAME").ok(),
            tool_version: std::env::var("PROV_TOOL_VERSION").ok(),
            editor: std::env::var("PROV_EDITOR").ok(),
            editor_version: std::env::var("PROV_EDITOR_VERSION").ok(),
            os: Some(std::env::consts::OS.to_string()),
        }
    }

    pub fn from_env() -> Self {
        Self {
            tool_name: std::env::var("PROV_TOOL_NAME").ok(),
            tool_version: std::env::var("PROV_TOOL_VERSION").ok(),
            editor: std::env::var("PROV_EDITOR").ok(),
            editor_version: std::env::var("PROV_EDITOR_VERSION").ok(),
            os: Some(std::env::consts::OS.to_string()),
        }
    }
}

// The main provenance struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub generator: Generator,
    pub timestamp: DateTime<Utc>,
    pub model: Option<ModelInfo>,
    pub environment: Option<Environment>,

    // Hash of the prompt or input that led to this output.
    pub prompt_hash: Option<String>,

    // Optional short excertp of the prompt (for human readability) 
    pub prompt_excerpt: Option<String>,

    // Optional list of external sources (URLs, docs, etc.).
    pub sources: Vec<String>,

    // Optional free-form tags or extensions.
    pub tags: HashMap<String, String>,
}


