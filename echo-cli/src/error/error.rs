/// The code is defining an error enum called `CliError` using the `thiserror` crate. The `thiserror`
/// crate provides a convenient way to define custom error types in Rust.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Path does not exist")]
    PathDoesNotExist,

    #[error("Duplicate config file exists")]
    DuplicateFileFound,

    #[error("Config file extension should be echo")]
    InvalidConfigFileExtension,
}
