use crate::command::Commands;
use indicatif::ProgressBar;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::cli::CLI;

use std::process;

/// The function `build_wasm` builds a WebAssembly module using a set of configuration files.
pub fn build_wasm(args: &CLI) {
    let mut progress_bar = ProgressBar::new(100);

    let mut composer = composer::Composer::default();
    let current_path = std::env::current_dir().unwrap();

    match &args.commands {
        Commands::Build(build) => {
            let mut configs = HashSet::new();

            for config_file in build.config.iter() {
                let config_path = PathBuf::from(config_file);
                progress_bar.inc((12 / build.config.len()).try_into().unwrap());

                let config_str = if !config_path.is_absolute() {
                    let combined_path = current_path.join(config_path.clone());

                    if let Ok(absolute_path) = combined_path.canonicalize() {
                        absolute_path.to_str().unwrap().to_string()
                    } else {
                        eprintln!("Error: The path does not exist");
                        process::exit(1);
                    }
                } else {
                    config_path.to_str().unwrap().to_string()
                };

                if !configs.insert(config_str.clone()) {
                    eprintln!("Error: Duplicate config file found: {}", config_str);
                    process::exit(1); // Exit with an error code
                }

                composer.add_config(&config_str);
                progress_bar.inc((12 / build.config.len()).try_into().unwrap());
            }

            composer.generate_wasm(args.verbose, &mut progress_bar).unwrap();
            progress_bar.finish_with_message("msg");
        }
    }
}