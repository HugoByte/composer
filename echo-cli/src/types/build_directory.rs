use std::collections::HashSet;
use std::path::PathBuf;

use clap_builder::Parser;
use indicatif::ProgressBar;
use crate::command::Commands;

use crate::cli::CLI;
use std::process;

/// The function `build_wasm` builds a WebAssembly module using a set of configuration files.
pub fn build_wasm() {

    let args = CLI::parse();
    let mut progress_bar = ProgressBar::new(100);

    let mut composer = composer::Composer::default();
    let current_path = std::env::current_dir().unwrap();

    if let Commands::Build(build) = args.commands {
        let mut configs = HashSet::new();

        for config_file in build.config.iter() {
            let config_path = PathBuf::from(config_file);
            progress_bar.inc(5);

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
            progress_bar.inc(5);
        }

        composer.generate(args.verbose, &mut progress_bar).unwrap();
        progress_bar.finish_with_message("msg");
    }

}