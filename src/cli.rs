use crate::{
    error::Result,
    generator::{Generator, GeneratorContext, ProjectScaffolder},
    utils::{ExecuterConfig, execute_command, run_with_spinner},
};
use clap::{Args, Parser, Subcommand};

/// Flus - Scaffolding the flutter project basic structure and features
#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    commands: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    /// Create new scaffolded project
    Create(CreateCommand),
    /// Generate different features
    Generate,
}

#[derive(Debug, Args)]
struct CreateCommand {
    project_name: String,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.commands {
            CliCommand::Create(command) => {
                handle_create_command(command)?;
            }
            CliCommand::Generate => {
                handle_generate_command();
            }
        };
        Ok(())
    }
}

fn handle_create_command(command: CreateCommand) -> Result<()> {
    let config = ExecuterConfig::default();
    let project_base_path = config.base_path.clone();
    run_with_spinner("Creating flutter project", || {
        execute_command("flutter", &["create", &command.project_name], config)
    })?;
    println!("Project created successfully");

    run_with_spinner("Creating basic folder structure", || {
        let base_path = project_base_path.join(&command.project_name);
        let project_scaffolder = ProjectScaffolder;
        let context = GeneratorContext::new(command.project_name, base_path);
        project_scaffolder.run(&context)
    })?;
    println!("Created basic folder structure");
    Ok(())
}

fn handle_generate_command(){
    println!("Notice: For now generating the features is not supported and is working on it");
}