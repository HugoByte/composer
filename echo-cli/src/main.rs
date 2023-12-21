use clap::Parser;
use echo_cli::cli::CLI;
use echo_cli::command::Commands;
use echo_cli::error::error::CliError;
use echo_cli::types::build_wasm;
use std::fs;
use std::path::Path;

/// The main function checks if the given configuration files exist and have the correct extension, and
/// then proceeds to generate a wasm file.
///
/// Returns:
///
/// The main function is returning a Result type with the Ok variant if the function executes
/// successfully, and the Err variant if there is an error. The Ok(()) value indicates that the function
/// returns a unit type, which means it doesn't return any meaningful value.
fn main() -> Result<(), CliError> {
    let args = CLI::parse();

    let Commands::Build(build) = &args.commands;
    for path in &build.config {
        if let Some(extension) = Path::new(path).extension() {
            if extension != "echo" {
                return Err(CliError::InvalidConfigFileExtension);
            }
        } else {
            return Err(CliError::PathDoesNotExist);
        }

        // Checking if the information of file exists or not
        if let Ok(metadata) = fs::metadata(path) {
            if !metadata.is_file() {
                eprintln!("Error: Path is not a regular file: {}", path);
                continue;
            }
        }
    }
    // Generate wasm file
    let result = build_wasm(&args);
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
