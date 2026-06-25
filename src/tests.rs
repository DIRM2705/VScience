#[cfg(test)]
pub mod tests
{
    use crate::environment::Environment;

    #[test]
    fn test_environment_creation() {
        let project_dir = std::path::PathBuf::from("./test_project_dir");

        let env = Environment::new(&project_dir);

        assert!(env.is_ok());

        let env = env.ok().unwrap();
        let venv_result = env.make_environment();
        assert!(venv_result.is_ok());
        assert!(project_dir.join(".venv").exists());
        let structure_result = env.make_structure();
        assert!(structure_result.is_ok());
        assert!(project_dir.join("src").exists());
        assert!(project_dir.join("data").exists());
        assert!(project_dir.join("models").exists());
        assert!(project_dir.join("experiments").exists());
    }

    #[test]
    fn test_environment_no_existing_directory() {
        let project_dir = std::path::PathBuf::from("./non_existent_dir");

        println!("Project directory: {:?}", project_dir);
        println!("Does the project directory exist? {}", project_dir.exists());
        println!("Is the project directory a directory? {}", project_dir.is_dir());

        let env = Environment::new(&project_dir);
        assert!(env.is_err());
    }

    #[test]
    fn test_pyproject_toml() {
        let project_dir = std::path::PathBuf::from("./test_project_dir");


        let env = Environment::new(&project_dir).ok().unwrap();
        let result = env.generate_ml_pyproject_toml("test_project", vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pyproject_toml_invalid_name() {
        let project_name = "invalid project name!";
        let project_dir = std::path::PathBuf::from("./test_project_dir");

        let env = Environment::new(&project_dir).ok().unwrap();

        let result = env.generate_ml_pyproject_toml(project_name, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_pyproject_toml_long_name() {
        let project_name = "a_very_long_project_name_that_exceeds_typical_length_limits_for_testing_purposes";
        let project_dir = std::path::PathBuf::from("./test_project_dir");

        let env = Environment::new(&project_dir).ok().unwrap();
        let result = env.generate_ml_pyproject_toml(project_name, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_pyproject_toml_no_name() {
        let project_name = "";
        let project_dir = std::path::PathBuf::from("./test_project_dir");

        let env = Environment::new(&project_dir).ok().unwrap();

        let result = env.generate_ml_pyproject_toml(project_name, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_existing_structure() {
        let project_dir = std::path::PathBuf::from("./test_project_dir");

        // Create an existing structure for testing
        let env = Environment::new(&project_dir).ok().unwrap();
        let validation_result = env.verify_environment();
        assert!(validation_result.is_err());

        // Clean up after the test
        std::fs::remove_dir_all(&project_dir).unwrap();
    }
}