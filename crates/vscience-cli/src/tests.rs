#[cfg(test)]
pub mod tests {

    use std::sync::Once;

    use crate::environment::Environment;

    static INIT: Once = Once::new();
    static TEMP_DIR: &str = "./test_project_dir";

    fn setup() {
        INIT.call_once(|| {
            println!("Setting up test environment...");
            let project_dir = std::path::PathBuf::from(TEMP_DIR);
            if !project_dir.exists() {
                std::fs::create_dir_all(TEMP_DIR).unwrap();

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

                let env = Environment::new(&project_dir).ok().unwrap();
                let result = env.generate_ml_pyproject_toml("test_project", vec![]);
                assert!(result.is_ok());
            }
        });
    }

    #[test]
    fn test_environment_no_existing_directory() {
        setup();
        let project_dir = std::path::PathBuf::from("./non_existent_dir");

        println!("Project directory: {:?}", project_dir);
        println!("Does the project directory exist? {}", project_dir.exists());
        println!(
            "Is the project directory a directory? {}",
            project_dir.is_dir()
        );

        let env = Environment::new(&project_dir);
        assert!(env.is_err());
    }


    #[test]
    fn test_pyproject_toml_invalid_name() {
        setup();
        let project_name = "invalid project name!";
        let project_dir = std::path::PathBuf::from(TEMP_DIR);

        let env = Environment::new(&project_dir).ok().unwrap();

        let result = env.generate_ml_pyproject_toml(project_name, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_pyproject_toml_long_name() {
        setup();
        let project_name =
            "a_very_long_project_name_that_exceeds_typical_length_limits_for_testing_purposes";
        let project_dir = std::path::PathBuf::from(TEMP_DIR);

        let env = Environment::new(&project_dir).ok().unwrap();
        let result = env.generate_ml_pyproject_toml(project_name, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_pyproject_toml_no_name() {
        setup();
        let project_name = "";
        let project_dir = std::path::PathBuf::from(TEMP_DIR);

        let env = Environment::new(&project_dir).ok().unwrap();

        let result = env.generate_ml_pyproject_toml(project_name, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_existing_structure() {
        setup();
        let project_dir = std::path::PathBuf::from(TEMP_DIR);

        // Create an existing structure for testing
        let env = Environment::new(&project_dir).ok().unwrap();
        let validation_result = env.verify_environment();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_adding_and_removing_dependency() {
        setup();
        let project_dir = std::path::PathBuf::from(TEMP_DIR);

        // Create an environment without a .venv directory
        let env = Environment::new(&project_dir).ok().unwrap();
        let validation_result = env.verify_environment();
        if validation_result.is_ok()
        // The .venv directory doesn't exist, create it
        {
            let venv_result = env.make_environment();
            assert!(venv_result.is_ok());
        }

        let add_result = env.add_package(&Some("numpy".to_string()), &None, &None);
        assert!(add_result.is_ok());

        let remove_result = env.remove_package("numpy");
        assert!(remove_result.is_ok());
    }

    #[test]
    fn test_add_dependency_without_venv() {
        setup();
        let project_dir = std::path::PathBuf::from("./non_venv_dir");
        
        std::fs::create_dir_all(&project_dir).unwrap();

        // Create an environment without a .venv directory
        let env = Environment::new(&project_dir).ok().unwrap();
        let validation_result = env.verify_environment();
        assert!(validation_result.is_ok()); // The .venv directory doesn't exist

        let add_result = env.add_package(&Some("numpy".to_string()), &None, &None);
        assert!(add_result.is_err());

        std::fs::remove_dir_all(&project_dir).unwrap(); // Clean up
    }
}
