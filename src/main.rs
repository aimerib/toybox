use color_eyre::eyre::Result;

use clap::{Parser, Subcommand};

mod commands;
mod utils;
use commands::new;

#[derive(Parser)]
#[command(author = "Aimeri Baddouh")]
#[command(version)]
#[command(about = "Toybox is a project management tool for the Playdate system sdk.")]
#[command(long_about = "Toybox is a project management tool for the Playdate system sdk.\nIt is designed to make it easy to create and manage projects for the Playdate.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new project in the current directory
    New {
        /// The name of the project. If not provided, the current directory name will be used.
        name: Option<String>,
    },
    /// Builds the project using the installed SDK
    Build,
    /// Builds and runs the project using the installed SDK
    Run,
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => new::new_project(name.clone())?,
        Commands::Build => commands::build::build_project()?,
        Commands::Run => commands::run::run_project()?,
    };
    Ok(())
}
