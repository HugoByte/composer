use crate::command::Commands;
use crate::error::CliError;
use indicatif::ProgressBar;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::cli::CLI;

/// The function `build_wasm` takes command line arguments and builds a WebAssembly module based on the
/// provided configuration files.
///
/// Arguments:
///
/// * `args`: The `args` parameter is of type `&CLI`, which is a reference to a struct representing
/// command line arguments.
///
/// Returns:
///
/// The function `build_wasm` returns a `Result` enum with the success case containing a `String`
/// indicating that the Wasm was generated successfully, and the error case containing a `CliError`
/// indicating the reason for failure.
pub fn build_wasm(args: &CLI) -> Result<String, CliError> {
    let mut progress_bar = ProgressBar::new(100);
    let mut composer = composer::Composer::default();
    let current_path = std::env::current_dir().map_err(|_| CliError::PathDoesNotExist)?;

    match &args.commands {
        Commands::Build(build) => {
            let mut configs = HashSet::new();

            for config_file in &build.config {
                let config_path = PathBuf::from(config_file);
                let absolute_path = if !config_path.is_absolute() {
                    current_path.join(&config_path).canonicalize()
                } else {
                    config_path.canonicalize()
                }
                .map_err(|_| CliError::PathDoesNotExist)?;

                let config_str = absolute_path
                    .to_str()
                    .ok_or(CliError::PathDoesNotExist {})?;

                if !configs.insert(config_str.to_string()) {
                    return Err(CliError::DuplicateFileFound);
                }

                composer.add_config(config_str);
                progress_bar.inc(12 / build.config.len() as u64);
            }

            match composer.generate_wasm(args.verbose, &mut progress_bar) {
                Ok(_) => {
                    progress_bar.finish_with_message("Wasm generated");
                    Ok("Wasm generated successfully".to_string())
                }
                Err(err) => {
                    Err(CliError::Error { err })
                }
            }
        }
    }
}
