use std::{fmt, io, path::Path};

#[derive(Debug, Clone)]
pub struct Error {
    description: String,
}

impl Error {
    #[cold]
    pub fn new(description: String) -> Self {
        Self { description }
    }

    #[cold]
    pub fn with_file_reference(path: &Path, line_number: usize, description: &str) -> Self {
        Self::new(format!(
            "{}:{}: {}",
            path.display(),
            line_number,
            description
        ))
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self {
            description: error.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
