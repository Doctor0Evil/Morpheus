use crate::config::OrchestratorConfig;
use crate::interpreter::interpret_spec_file;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Debug, Parser)]
#[command(name = "morpheus-orchestrator")]
#[command(about = "Expose Morpheus neuromorph governance to CI and orchestrators")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Load ALN spec and emit governance profile as JSON
    Eval {
        #[arg(short, long)]
        spec: Option<String>,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Eval { spec } => {
            let cfg = OrchestratorConfig::from_env_or_default();
            let path = spec.map(std::path::PathBuf::from).unwrap_or(cfg.spec_path);
            let profile = interpret_spec_file(&path)?;
            let json = serde_json::to_string_pretty(&profile)?;
            println!("{json}");
        }
    }
    Ok(())
}
