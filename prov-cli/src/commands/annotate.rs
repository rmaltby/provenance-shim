use clap::Args;
use provenance_core::{
    Provenance, Generator, Environment, ModelInfo,
    hash_prompt, annotate_text,
};
use std::fs;

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
        let input = match &self.file {
            Some(path) => fs::read_to_string(path)?,
            None => {
                use std::io::Read;
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                buf
            }
        };

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
        
        let annotated = annotate_text(&input, &prov)
            .map_err(|e| anyhow::anyhow!(e))?;

        println!("{annotated}");
        Ok(())
    }
}