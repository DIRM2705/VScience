mod environment;
mod tests;

use environment::*;
use tests::*;
use std::env;
use std::path::Path;

pub fn init(project_name: &str, project_dir: &str) {
    /*
    Creates the virtual environment and the project structure for the ML project
    */

    let project_dir = Path::new(project_dir);

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }

    let cmd = args[1].as_str();
    match cmd {
        "init" => {
            let mut project_name = args[2].as_str();
            let mut project_dir = args[3].as_str();

            if project_name.is_empty()
            {
                project_name = "my_project";
            }

            if project_dir.is_empty()
            {
                project_dir = ".";
            }

            init(project_name, project_dir);
        }
        "help" => {
            help();
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
        }
        
    }
}
