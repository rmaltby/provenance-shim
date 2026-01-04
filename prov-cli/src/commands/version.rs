use clap::Args;

#[derive(Args)]
pub struct VersionArgs {}

impl VersionArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("prov-cli {}", env!("CARGO_PKG_VERSION"));
        println!("License: GPL-3.0-or-later OR Apache-2.0");
        Ok(())
    }
}
