mod commands;
mod environment;
mod tests;

use clap::Parser;
use commands::*;
use environment::*;
use inquire::{MultiSelect, formatter::MultiOptionFormatter};
use std::fs::{remove_dir_all};
use std::path::PathBuf;

struct PromptingError;

impl std::fmt::Display for PromptingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to get user input for dependencies")
    }
}

fn select_dependencies() -> Result<Vec<&'static str>, PromptingError> {
    /*
    Prompts the user to select dependencies for the project
    */
    let dependencies = vec![
        "numpy",
        "pandas",
        "polars",
        "scikit-learn",
        "matplotlib",
        "seaborn",
        "tensorflow",
        "torch",
        "xgboost",
        "lightgbm",
        "catboost",
    ];

    let formatter: MultiOptionFormatter<'_, &str> = &|a| format!("{} dependencies", a.len());
    let answer = MultiSelect::new("Select dependencies for the project", dependencies)
        .with_formatter(formatter)
        .prompt();

    if answer.is_err() {
        return Err(PromptingError);
    }

    Ok(answer.unwrap())
}

fn should_remove_existing_project() -> Result<bool, PromptingError> {
    /*
    Prompts the user to confirm if they want to remove the existing project
    */
    let answer = inquire::Confirm::new("A project already exists. Do you want to delete it?")
        .with_default(false)
        .prompt();

    return match answer {
        Ok(truth_value) => Ok(truth_value),
        Err(_) => Err(PromptingError),
    };
}

pub fn init(project_name: &str, project_dir: &PathBuf) {
    /*
    Creates the virtual environment and the project structure for the ML project
    */

    let env = Environment::new(project_dir);

    if env.is_err() {
        eprint!("{}", env.err().unwrap());
        return;
    }

    let env = env.ok().unwrap();

    if env.verify_environment().is_err() {
        let answer = should_remove_existing_project();

        match answer {
            Ok(true) => {
                let remove_result = remove_dir_all(project_dir);

                if remove_result.is_err() {
                    eprint!("Failed to remove existing project directory.");
                    return;
                }
            }
            Ok(false) => {
                println!("Aborting the initialization process.");
                return;
            }
            Err(e) => {
                eprint!("{}", e);
                return;
            }
        }
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
    let selected_dependencies = match select_dependencies() {
        Ok(deps) => deps,
        Err(e) => {
            eprint!("{}", e);
            return;
        }
    };

    if let Err(e) = env.generate_ml_pyproject_toml(project_name, selected_dependencies) {
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

#[derive(Parser)]
#[command(name = "vsci")]
#[command(about = "A tool for creating, managing and documenting ML projects.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {
            project_name,
            project_dir,
        }) => {
            let project_name_arg = project_name
                .clone()
                .unwrap_or(String::from("my-ml-project"));
            let project_dir_arg = project_dir.clone().unwrap_or_else(|| PathBuf::from("."));
            init(&project_name_arg, &project_dir_arg);
        },
        
        None => {
            eprintln!("No command provided. Use --help for more information.");
        }
    }
}
