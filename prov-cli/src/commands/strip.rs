use clap::Args;
use provenance_core::strip_provenance;
use std::fs;

#[derive(Args)]
pub struct StripArgs {
    /// File to strip provenance from (optional; otherwise reads from stdin)
    #[arg(short, long)]
    pub file: Option<String>,
}

impl StripArgs {
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

        let stripped = strip_provenance(&input);
        println!("{stripped}");
        Ok(())
    }
}