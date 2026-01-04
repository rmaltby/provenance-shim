// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use crate::model::{Provenance, Generator, ModelInfo, Environment};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

pub struct ProvenanceBuilder {
    generator: Generator,
    timestamp: DateTime<Utc>,
    model: Option<ModelInfo>,
    environment: Option<Environment>,
    prompt_hash: Option<String>,
    prompt_excerpt: Option<String>,
    sources: Vec<String>,
    tags: HashMap<String, String>,
}

impl ProvenanceBuilder {
    pub fn new(generator: Generator) -> Self {
        Self{
            generator,
            timestamp: Utc::now(),
            model: None,
            environment: None, 
            prompt_hash: None,
            prompt_excerpt: None,
            sources: Vec::new(),
            tags: HashMap::new(),
        }
    }

    pub fn model(mut self, model: ModelInfo) -> Self {
        self.model = Some(model);
        self
    }

    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = Some(env);
        self
    }

    pub fn prompt_hash(mut self, hash: impl Into<String>) -> Self {
        self.prompt_hash = Some(hash.into());
        self
    }

    pub fn prompt_excerpt(mut self, excerpt: impl Into<String>) -> Self {
        self.prompt_excerpt = Some(excerpt.into());
        self
    }

    pub fn add_source(mut self, src: impl Into<String>) -> Self {
        self.sources.push(src.into());
        self
    }

    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Provenance {
        Provenance {
            generator: self.generator,
            timestamp: self.timestamp,
            model: self.model,
            environment: self.environment,
            prompt_hash: self.prompt_hash,
            prompt_excerpt: self.prompt_excerpt,
            sources: self.sources,
            tags: self.tags,
        }
    }
}

impl Provenance {
    pub fn builder(generator: Generator) -> ProvenanceBuilder {
        ProvenanceBuilder::new(generator)
    }
}