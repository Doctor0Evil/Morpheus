use clap::{Parser, Subcommand};
use morpheus_neuromorph_core::MorpheusEngine;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "morpheus-cli")]
#[command(about = "Morpheus_GPT neuromorphic core control interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show default Morpheus_GPT provider configuration
    ShowConfig,
    /// Export active endpoints as JSON
    ExportEndpoints,
    /// Test a non-reversal action
    EnforceAction {
        /// Action name (e.g., upgrade, rollback)
        action: String,
    },
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let engine = MorpheusEngine::new().expect("failed to initialize MorpheusEngine");

    match cli.command {
        Commands::ShowConfig => {
            let json = engine
                .ctx
                .provider_config
                .as_json()
                .expect("config serialization failed");
            println!("{json}");
        }
        Commands::ExportEndpoints => {
            engine.register_example_endpoints();
            let json = engine.export_active_endpoints_json();
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Commands::EnforceAction { action } => match engine.enforce_no_reversal(&action) {
            Ok(_) => {
                println!("Action '{action}' is allowed for neuromorphic evolution.");
            }
            Err(e) => {
                eprintln!("Action '{action}' is not allowed: {e}");
                std::process::exit(1);
            }
        },
    }
}
