mod errors;

use errors::*;
use std::fs::{create_dir_all, write, File};
use std::path::PathBuf;
use std::process::Command;
use toml_edit::{Array, DocumentMut, Item, Table, Value};
use which::which;

pub struct Environment {
    path: PathBuf,
}

impl Environment {
    pub fn new(project_dir: &PathBuf) -> Result<Self, Box<dyn EnvironmentError>> {
        /*
        Creates a new Environment instance with the given path
        */

        if !project_dir.exists() || !project_dir.is_dir() {
            return Err(Box::new(EnvironmentCreationError::new(
                "The specified project directory does not exist or is not a directory.",
            )));
        }

        let env = Environment {
            path: project_dir.clone(),
        };
        return Ok(env);
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
            return Err(Box::new(EnvironmentCreationError::new(
                "The virtual environment already exists. Please remove it before proceeding.",
            )));
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
            return Err(DirCreationError::from_error(
                &data_root.to_string_lossy(),
                Box::new(e),
            ));
        }

        let processed_dir_creation_result = create_dir_all(&data_root.join("processed"));
        if let Err(e) = processed_dir_creation_result {
            return Err(DirCreationError::from_error(
                &data_root.join("processed").to_string_lossy(),
                Box::new(e),
            ));
        }

        let raw_dir_creation_result = create_dir_all(&data_root.join("raw"));
        if let Err(e) = raw_dir_creation_result {
            return Err(DirCreationError::from_error(
                &data_root.join("raw").to_string_lossy(),
                Box::new(e),
            ));
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
            return Err(Box::new(DirCreationError::from_error(
                &src_root.to_string_lossy(),
                Box::new(e),
            )));
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

        let models_root = self.path.join("models");
        let models_dir_creation_result = create_dir_all(&models_root);
        if let Err(e) = models_dir_creation_result {
            return Err(Box::new(DirCreationError::from_error(
                &models_root.to_string_lossy(),
                Box::new(e),
            )));
        }

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

    fn project_name_is_valid(&self, project_name: &str) -> bool {
        /*
        Validates the project name
        */
        return !project_name.is_empty()
            && project_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            && project_name.chars().next().unwrap_or_default().is_alphabetic() // First character must be alphabetic
            && project_name.len() <= 50;
    }

    pub fn generate_ml_pyproject_toml(
        &self,
        project_name: &str,
        selected_dependencies: Vec<&str>,
    ) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Generates the pyproject.toml file for the ML project
        */

        if !self.project_name_is_valid(project_name) {
            return Err(Box::new(EnvironmentCreationError::new(
                "Project name must be non-empty and can only contain alphanumeric characters, hyphens, and underscores.",
            )));
        }

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

        let mut dependencies = Array::new();
        for dep in selected_dependencies {
            dependencies.push(Value::from(dep));
        }
        project.insert("dependencies", Value::Array(dependencies).into());

        doc.insert("project", Item::Table(project));

        // Modern uv workspace
        let mut dependency_groups = Table::new();
        dependency_groups.insert(
            "dev",
            Item::Value(Value::Array(vec!["uv", "pytest"].into_iter().collect())).into(),
        );
        doc.insert("dependency-groups", Item::Table(dependency_groups));

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

    pub fn add_package(
        &self,
        package_name: &Option<String>,
        requirements_file: &Option<PathBuf>,
        constraints_file: &Option<PathBuf>,
    ) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Adds a package to the project using uv
        */
        if which("uv").is_err() {
            return Err(Box::new(UVMissingError));
        }

        if !self.path.join(".venv").exists() {
            return Err(Box::new(EnvironmentCreationError::new(
                "The virtual environment does not exist. Please create it before adding packages.",
            )));
        }

        let mut add_cmd = &mut Command::new("uv");
        add_cmd = add_cmd.arg("add");

        if let Some(package_name) = package_name {
            add_cmd = add_cmd.arg(package_name);
        }

        if let Some(requirements_file) = requirements_file {
            add_cmd = add_cmd.arg("-r").arg(requirements_file);
        }

        if let Some(constraints_file) = constraints_file {
            add_cmd = add_cmd.arg("-c").arg(constraints_file);
        }

        let cmd_status_result = add_cmd.current_dir(&self.path).status();

        if cmd_status_result.is_err() {
            return Err(Box::new(UVFailedError::new(
                "Failed to run add package command.",
            )));
        }

        let add_status = cmd_status_result.unwrap();

        if !add_status.success() {
            return Err(Box::new(UVFailedError::new(
                "Failed to add package using uv.",
            )));
        }

        return Ok(());
    }

    pub fn remove_package(&self, package_name: &str) -> Result<(), Box<dyn EnvironmentError>> {
        /*
        Removes a package from the project using uv
        */
        if which("uv").is_err() {
            return Err(Box::new(UVMissingError));
        }
        
        if !self.path.join(".venv").exists() {
            return Err(Box::new(EnvironmentCreationError::new(
                "The virtual environment does not exist. Please create it before removing packages.",
            )));
        }

        let remove_cmd = Command::new("uv").arg("remove").arg(package_name).current_dir(&self.path).status();

        if remove_cmd.is_err() {
            return Err(Box::new(UVFailedError::new(
                "Failed to run remove package command.",
            )));
        }

        let remove_status = remove_cmd.unwrap();

        if !remove_status.success() {
            return Err(Box::new(UVFailedError::new(
                "Failed to remove package using uv.",
            )));
        }

        return Ok(());
    }
}
