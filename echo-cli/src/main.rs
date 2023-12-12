use clap::Parser;
use echo_cli::cli::CLI;
use echo_cli::types::build_wasm;
use std::fs;
use std::path::Path;
use echo_cli::command::Commands;


fn main() {
    let args = CLI::parse();

    if let Commands::Build(build) = args.commands {
        for path in &build.config {
            if let Some(extension) = Path::new(path).extension() {
                if extension != "echo" {
                    eprintln!("Error: Config file extension must be .echo: {}", path);
                    continue;
                }
            } else {
                eprintln!("Error: Invalid path format: {}", path);
                continue;
            }

            // Checking if the information of file exists or not
            if let Ok(metadata) = fs::metadata(path) {
                if !metadata.is_file() {
                    eprintln!("Error: Path is not a regular file: {}", path);
                    continue;
                }
            } else {
                eprintln!("Error: No such file or directory: {}", path);
                continue;
            }
        }
        // Generate wasm file
        build_wasm();
    }
}
