use super::*;

use crate::command::build;
use clap_builder::Parser;
use std::{cell::RefCell, collections::HashMap};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use build::Build;
use composer::Composer;

use crate::{cli::CLI, command::Commands, types::build_wasm};

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_build_struct() {
        let build_obj = build::Build {
            config: vec!["config1".to_string(), "config2".to_string()],
            output: None,
        };
        assert_eq!(
            build_obj.config,
            vec!["config1".to_string(), "config2".to_string()]
        );
        assert_eq!(build_obj.output, None);
    }

    #[test]
    #[should_panic]
    fn test_build_wasm_with_invalid_config() {
        let cli_args = CLI {
            commands: Commands::Build(Build {
                config: vec![
                    "/path/to/inavlid/config1".to_string(),
                    "/path/to/invalid/config2".to_string(),
                ],
                output: None,
            }),
            verbose: true,
        };

        build_wasm(&cli_args);
    }

    #[test]
    fn test_config_file_present() {
        let composer = Composer {
            config_files: vec!["config1".to_string(), "config2".to_string()],
            workflows: RefCell::new(Vec::new()),
            custom_types: RefCell::new(HashMap::new()),
        };

        assert!(composer.config_files.contains(&"config1".to_string()));
        assert!(composer.config_files.contains(&"config2".to_string()));
    }

    #[test]
    fn test_valid_commands() {
        let args = vec!["echo-cli", "build", "--config", "config1.echo"];

        let cli = CLI::parse_from(args);

        assert!(matches!(cli.commands, Commands::Build(_)));
    }

    #[test]
    #[should_panic]
    fn test_build_wasm_with_invalid_config_file_extension() {
        let args = CLI {
            commands: Commands::Build(Build {
                config: vec!["../config/car_market_place.star".to_string()],
                output: None,
            }),
            verbose: false,
        };
        build_wasm(&args);
    }
}
