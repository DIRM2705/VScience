mod environment;
mod tests;
mod commands;

use environment::*;
use std::path::PathBuf;
use commands::*;
use clap::Parser;

pub fn init(project_name: &str, project_dir: &PathBuf) {
    /*
    Creates the virtual environment and the project structure for the ML project
    */

    let env = Environment::new(project_dir);

    if let Err(e) = env.verify_environment() {
        eprint!("{}", e);
        return;
    }

    println!("Creating virtual environment...");
    if let Err(e) = env.make_environment() {
        eprint!("{}", e);
        return;
    }
    println!("Virtual environment created successfully.");

    println!("Creating project structure...");
    if let Err(e) = env.make_structure() {
        eprint!("{}", e);
        return;
    }
    println!("Project structure created successfully.");

    println!("Generating pyproject.toml...");
    if let Err(e) = env.generate_ml_pyproject_toml(project_name) {
        eprint!("{}", e);
        return;
    }
    println!("pyproject.toml generated successfully.");

    println!("Syncing virtual environment...");
    if let Err(e) = env.sync_status() {
        eprint!("{}", e);
        return;
    }
    println!("Virtual environment synced successfully.");
}

pub fn help()
{
    /* Displays help information */
    println!("Usage: vscience <command> [<arguments>...]");
    println!("Commands:");
    println!("  init <project_name> <project_dir>  Creates a new ML project with the given name and directory.");
    println!("  help                               Displays this help message.");
}

#[derive(Parser)]
#[command(name = "vscience")]
#[command(about = "A tool for creating, managing and documenting ML projects.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { project_name, project_dir }) => {
            let project_name_arg = project_name.clone().unwrap_or(String::from("my-ml-project"));
            let project_dir_arg = project_dir.clone().unwrap_or_else(|| PathBuf::from("."));
            init(&project_name_arg, &project_dir_arg);
        }
        None => {
            help();
        }
    }
}
