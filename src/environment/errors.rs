use std::{error::Error, fmt};

pub trait EnvironmentError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl fmt::Display for dyn EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

pub struct PromptingError;

impl EnvironmentError for PromptingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to get user input for dependencies.")
    }
}

pub struct UVFailedError
{
    error_msg : String
}

impl UVFailedError {
    pub fn new(msg : &str) -> Self {
        UVFailedError {
            error_msg: msg.to_string()
        }
    }
}

impl EnvironmentError for UVFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to create virtual environment using uv. Error: {}", self.error_msg)
    }
}

pub struct UVMissingError;

impl EnvironmentError for UVMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UV is not installed. Please install it to continue.")
    }
}

pub struct DirCreationError
{
    dir_name : String,
    error_msg : String
}

impl DirCreationError {
    pub fn from_error(dir_name : &str, error : Box<dyn Error>) -> Self {
        DirCreationError {
            dir_name: dir_name.to_string(),
            error_msg: error.to_string()
        }
    }
}

pub struct FileWriteError
{
    file_name : String,
    error_msg : String
}

impl EnvironmentError for DirCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to create directory '{}'. Error: {}", self.dir_name, self.error_msg)
    }
}

impl FileWriteError {
    pub fn from_error(file_name : &str, error : Box<dyn Error>) -> Self {
        FileWriteError {
            file_name: file_name.to_string(),
            error_msg: error.to_string()
        }
    }
}

impl EnvironmentError for FileWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to write to file '{}'. Error: {}", self.file_name, self.error_msg)
    }
}

impl From<DirCreationError> for Box<dyn EnvironmentError> {
    fn from(error: DirCreationError) -> Self {
        Box::new(error)
    }
}

impl From<FileWriteError> for Box<dyn EnvironmentError> {
    fn from(error: FileWriteError) -> Self {
        Box::new(error)
    }
}