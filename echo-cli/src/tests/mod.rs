use super::*;

use clap_builder::Parser;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::command::build;

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_valid_config_file_extension() {
        let valid_path = "config.echo";
        assert!(Path::new(valid_path).extension().unwrap() == "echo");
    }

    #[test]
    fn test_invalid_path_format() {
        let invalid_path = "invalid_path";
        // Assert
        assert!(Path::new(invalid_path).extension().is_none());
    }

    #[test]
    fn test_invalid_config_file_extension() {
        let invalid_path = "config.yaml";
        assert!(Path::new(invalid_path).extension().unwrap() != "echo");
    }

    #[test]
    fn test_build_struct() {
        // let build = Commands::Build(Build { config: vec!["config1".to_string(), "config2".to_string()], output: None });
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
}
