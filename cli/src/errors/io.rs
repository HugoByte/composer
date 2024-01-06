use super::*;

#[derive(Error, Debug)]
pub enum IOError {
    PathNotFound,
    Anyhow(Error),
    Other(String),
    Std(std::io::Error),
}

pub fn io_error(err: std::io::Error) -> Box<dyn Exception> {
    Box::new(IOError::from(err))
}

impl Exception for IOError {
    fn code(&self) -> i32 {
        match self {
            IOError::PathNotFound => 1,
            IOError::Other(_) => 2,
            IOError::Anyhow(_) => 3,
            IOError::Std(_) => 4,
        }
    }
}

impl From<std::io::Error> for IOError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => IOError::PathNotFound,
            _ => IOError::Std(value),
        }
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError::PathNotFound => write!(f, "PathNotFound"),
            IOError::Anyhow(error) => write!(f, "{}", error),
            IOError::Other(error) => write!(f, "{}", error),
            IOError::Std(error) => write!(f, "{}", error),
        }
    }
}
