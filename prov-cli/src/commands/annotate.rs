use clap::Args;
use provenance_core::{
    Provenance, Generator, Environment, ModelInfo,
    hash_prompt,
    adapters::comment::adapter_for_path,
    chain_path_for, load_chain, save_chain,
};
use std::fs;
use std::path::PathBuf;


#[derive(Args)]
pub struct AnnotateArgs {
    /// File to annotate (optional; otherwise reads from stdin)
    #[arg(short, long)]
    pub file: Option<String>,

    /// Prompt text used to generate the content
    #[arg(short, long)]
    pub prompt: Option<String>,
}

impl AnnotateArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        // For now, require a file path for chain-based annotation
        let file_path = match &self.file {
            Some(path) => PathBuf::from(path),
            None => anyhow::bail!("--file is required for chain-based annotation"),
        };

        // Assume current dir is repo root for now
        let repo_root = std::env::current_dir()?;
        let chain_path = chain_path_for(&repo_root, &file_path);

        // Read file contents
        let input = fs::read_to_string(&file_path)?;

        // Get a comment adapter for this file.
        let adapter = adapter_for_path(&file_path);

        // Strip existing pointer comment, if present.
        let (stripped_input, _had_pointer) = adapter.strip_pointer_comment(&input);

        // Build new provenance entry.
        let prompt = self.prompt.clone().unwrap_or_default();
        let prov = Provenance::builder(Generator::AiAssistant)
            .environment(Environment::from_env())
            .model(ModelInfo {
                name: Some("unknown".into()),
                provider: None,
                version: None,
            })
            .prompt_hash(hash_prompt(&prompt))
            .prompt_excerpt(prompt.chars().take(80).collect::<String>())
            .build();
        
        // Update the chain.
        let mut chain = load_chain(&chain_path)?;
        chain.append(prov);
        save_chain(&chain_path, &chain)?;

        // Insert fresh pointer comment at the top.
        let rel_chain_path = chain_path
            .strip_prefix(&repo_root)
            .unwrap_or(&chain_path);
        let rel_chain_str = rel_chain_path.to_string_lossy();
        let pointer = adapter.pointer_comment(&rel_chain_str);
        
        let new_contents = format!("{pointer}\n{stripped_input}");

        // Write back to file.
        fs::write(&file_path, new_contents)?;

        Ok(())
    }
}