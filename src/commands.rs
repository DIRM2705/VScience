use std::path::PathBuf;
use clap::Subcommand;

fn verify_path(s : &str) -> Result<PathBuf, String>
{
    let path = PathBuf::from(s);
    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        Err(format!("The path '{}' does not exist or is not a directory.", s))
    }
}

#[derive(Subcommand)]
pub enum Commands
{
    /// Creates a new ML project with the given name and directory.
    Init
    { 
        #[arg(short='n', long, help = "Name of the project to be created")]
        project_name: Option<String>,

        #[arg(short='d', long, value_parser = verify_path, help = "Directory where the project will be created")]
        project_dir: Option<PathBuf>
    },

    Add
    {
        #[arg(short, long, help = "Name of the package to be added")]
        package_name: String,

        #[arg(short='r', long, help = "Requirements file to install packages from")]
        requirements_file: Option<PathBuf>,

        #[arg(short='c', long, help = "Constraints file path")]
        constraints_file: Option<PathBuf>
    },

    Remove
    {
        #[arg(short, long, help = "Name of the package to be removed")]
        package_name: String
    },
}

