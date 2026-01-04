// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use provenance_core::{
    Provenance, Generator, Environment, ModelInfo,
    hash_prompt, annotate_text
};

fn main() {
    let prompt = "Explain the difference between a trait and a struct in Rust.";
    let output = "A struct defines data while a trait defines behavior.";

    let prov = Provenance::builder(Generator::AiAssistant)
        .environment(Environment::from_env())
        .model(ModelInfo {
            name: Some("unknown".to_string()),
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