mod errors;

use errors::*;
use inquire::{MultiSelect, formatter::MultiOptionFormatter};
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all, write, remove_dir_all};
use std::process::{Command, exit};
use toml_edit::{Array, DocumentMut, Item, Table, Value};
use which::which;

pub struct Environment {
    path: PathBuf,
}

impl Environment {
    pub fn new(path: &Path) -> Self {
        Environment {
            path : path.to_path_buf()
        }
    }

    pub fn verify_environment(&self) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Verifies that the virtual environment is set up correctly
        */
        if which("uv").is_err() {
            return Err(Box::new(UVMissingError));
        }

        let venv_path = self.path.join(".venv");

        if venv_path.exists() {
            let answer = inquire::Confirm::new("A project already exists. Do you want to delete it?")
                .with_default(false)
                .prompt();

            match answer {
                Ok(true) => {
                    let remove_result = remove_dir_all(venv_path);

                    if remove_result.is_err() {
                        return Err(Box::new(UVFailedError::new(
                            "Failed to remove existing virtual environment.",
                        )));
                    }
                }
                Ok(false) => {
                    println!("Aborting the initialization process.");
                    exit(0);
                }
                Err(_) => {
                    return Err(Box::new(PromptingError));
                }
            }
        }

        return Ok(());
    }

    pub fn make_environment(&self) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Creates a new python virtual environment using uv tool
         */
        if which("uv").is_err() {
            return Err(Box::new(UVMissingError));
        }

        // Create the virtual environment using uv

        let environment_creation_cmd = Command::new("uv")
        .arg("venv")
        .current_dir(&self.path)
        .status();

        if environment_creation_cmd.is_err() {
            return Err(Box::new(UVFailedError::new("Failed to run uv command.")));
        }

        let environment_creation_status = environment_creation_cmd.unwrap();

        if !environment_creation_status.success() {
            return Err(Box::new(UVFailedError::new(
                "Failed to create virtual environment using uv.",
            )));
        }

        return Ok(());
    }

    fn make_data_dir(&self) -> Result<(), DirCreationError> {
        /*
        Creates the data directory for the project
        */
        let data_root = self.path.join("data");
        let data_dir_creation_result = create_dir_all(&data_root);
        if let Err(e) = data_dir_creation_result {
            return Err(DirCreationError::from_error(&data_root.to_string_lossy(), Box::new(e)));
        }

        let processed_dir_creation_result = create_dir_all(&data_root.join("processed"));
        if let Err(e) = processed_dir_creation_result {
            return Err(DirCreationError::from_error(&data_root.join("processed").to_string_lossy(), Box::new(e)));
        }

        let raw_dir_creation_result = create_dir_all(&data_root.join("raw"));
        if let Err(e) = raw_dir_creation_result {
            return Err(DirCreationError::from_error(&data_root.join("raw").to_string_lossy(), Box::new(e)));
        }

        return Ok(());
    }

    fn make_src_dir(&self) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Creates the src directory for the project
        */

        let src_root = self.path.join("src");

        let src_dir_creation_result = create_dir_all(&src_root);
        if let Err(e) = src_dir_creation_result {
            return Err(Box::new(DirCreationError::from_error(&src_root.to_string_lossy(), Box::new(e))));
        }

        // Create template files
        let init_file_creation_result = File::create(&src_root.join("__init__.py"));
        if let Err(e) = init_file_creation_result {
            return Err(Box::new(FileWriteError::from_error(
                &src_root.join("__init__.py").to_string_lossy(),
                Box::new(e),
            )));
        }

        let data_pipeline_file_creation_result = File::create(&src_root.join("data_pipeline.py"));
        if let Err(e) = data_pipeline_file_creation_result {
            return Err(Box::new(FileWriteError::from_error(
                &src_root.join("data_pipeline.py").to_string_lossy(),
                Box::new(e),
            )));
        }

        let train_file_creation_result = File::create(&src_root.join("train.py"));
        if let Err(e) = train_file_creation_result {
            return Err(Box::new(FileWriteError::from_error(
                &src_root.join("train.py").to_string_lossy(),
                Box::new(e),
            )));
        }

        let evaluate_file_creation_result = File::create(&src_root.join("evaluate.py"));
        if let Err(e) = evaluate_file_creation_result {
            return Err(Box::new(FileWriteError::from_error(
                &src_root.join("evaluate.py").to_string_lossy(),
                Box::new(e),
            )));
        }

        return Ok(());
    }

    pub fn make_structure(&self) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Creates the project structure for the virtual environment
        */

        self.make_data_dir()?;
        self.make_src_dir()?;

        let experiments_root = self.path.join("experiments");

        let experiments_dir_creation_result = create_dir_all(&experiments_root);
        if let Err(e) = experiments_dir_creation_result {
            return Err(Box::new(DirCreationError::from_error(
                &experiments_root.to_string_lossy(),
                Box::new(e),
            )));
        }
        return Ok(());
    }

    fn select_dependencies(&self) -> Result<Vec<&'static str>, Box<dyn EnvironmentError>> {
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
            return Err(Box::new(PromptingError));
        }

        Ok(answer.unwrap())
    }

    pub fn generate_ml_pyproject_toml(
        &self,
        project_name: &str,
    ) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Generates the pyproject.toml file for the ML project
        */

        let mut doc = DocumentMut::new();

        // Create METADATA.toml file
        let mut project = Table::new();
        project.insert("name", Value::from(project_name).into());
        project.insert("version", Value::from("0.1.0").into());
        project.insert("description", Value::from("No description yet.").into());
        project.insert(
            "authors",
            Value::Array(Array::from_iter(vec![Value::from(
                "Your Name <your.email@example.com>",
            )]))
            .into(),
        );
        project.insert("requires-python", Value::from(">=3.11").into());

        // Dependencies
        let selection = self.select_dependencies()?;
        let mut dependencies = Array::new();
        for dep in selection {
            dependencies.push(Value::from(dep));
        }
        project.insert("dependencies", Value::Array(dependencies).into());

        doc.insert("project", Item::Table(project));

        // Modern uv workspace
        let mut tool_uv = Table::new();
        tool_uv.insert(
            "dev-dependencies",
            Item::Value(Value::Array(vec!["uv", "pytest"].into_iter().collect())),
        );
        let mut tool = Table::new();
        tool.insert("uv", Item::Table(tool_uv));
        doc.insert("tool", Item::Table(tool));

        let file_path = self.path.join("pyproject.toml");
        let write_result = write(file_path, doc.to_string());
        if let Err(e) = write_result {
            return Err(Box::new(FileWriteError::from_error(
                "pyproject.toml",
                Box::new(e),
            )));
        }

        return Ok(());
    }

    pub fn sync_status(&self) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Syncs the status of the virtual environment
        */
        if which("uv").is_err() {
            return Err(Box::new(UVMissingError));
        }

        let sync_cmd = Command::new("uv")
            .arg("pip")
            .arg("install")
            .arg("-r")
            .arg("pyproject.toml")
            .current_dir(&self.path)
            .status();

        if sync_cmd.is_err() {
            return Err(Box::new(UVFailedError::new(
                "Failed to run syncronization command.",
            )));
        }

        let sync_status = sync_cmd.unwrap();

        if !sync_status.success() {
            return Err(Box::new(UVFailedError::new(
                "Failed to install dependencies using uv.",
            )));
        }

        return Ok(());
    }
}
