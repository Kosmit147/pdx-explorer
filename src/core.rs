use std::{fmt, io, path, path::Path, result};

pub type Result<T, E = Error> = result::Result<T, E>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Error::new(format!($($arg)*))
    };
}

pub use error;

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for Error {}

macro_rules! error_from_impl {
    ($t:ty) => {
        impl From<$t> for Error {
            fn from(error: $t) -> Self {
                Self::new(error.to_string())
            }
        }
    };
}

error_from_impl!(io::Error);
error_from_impl!(path::StripPrefixError);
error_from_impl!(rusqlite::Error);
