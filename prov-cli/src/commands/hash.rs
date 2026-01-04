use clap::Args;
use provenance_core::hash_prompt;

#[derive(Args)]
pub struct HashArgs {
    pub text: String,
}

impl HashArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let hash = hash_prompt(&self.text);
        println!("{hash}");
        Ok(())
    }
}