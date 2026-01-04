mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "prov", version, about = "Provenance annotation tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Annotate a file with provenance information
    Annotate(commands::annotate::AnnotateArgs),
    /// Strip provenance information from a file
    Strip(commands::strip::StripArgs),
    /// Compute the hash of a file
    Hash(commands::hash::HashArgs),
    /// Show the version of the tool
    Version(commands::version::VersionArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Annotate(args) => args.run(),
        Commands::Strip(args) => args.run(),
        Commands::Hash(args) => args.run(),
        Commands::Version(args) => args.run(),
    }
}