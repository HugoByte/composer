use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version = "1.0.0", about = "The echo-cli is a CLI tool used to generate the wasm binary files", long_about = None)]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
    #[structopt(
        long = "verbose",
        short,
        global = true,
        default_value = "false",
        help = "Suppress CLI output"
    )]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate(Generate),
    Test(Test),
}

#[derive(Parser, Debug)]
#[command(author, version = "1.0.0", about = "generate the wasm file from the given config file", long_about = None)]
struct Generate {
    #[clap(short, long, value_parser)]
    config: Vec<String>,
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version = "1.0.0", about = "Test the flow of the config file", long_about = None)]
struct Test {
    #[clap(short, long, value_parser)]
    config: Vec<String>,
}

fn generate_wasm() {
    let args = Args::parse();
    let mut pb = ProgressBar::new(100);

    let mut c = composer::Composer::default();
    let current_path = std::env::current_dir().unwrap(); // Replace with the current directory path

    if let Commands::Generate(generate) = args.commands {
        for config_file in generate.config.iter() {
            let config_path = PathBuf::from(config_file);
            pb.inc(5);
            if !config_path.is_absolute() {
                let combined_path = current_path.join(config_path.clone());

                if let Ok(absolute_path) = combined_path.canonicalize() {
                    c.add_config(absolute_path.to_str().unwrap());
                } else {
                    print!("error");
                }
            } else {
                c.add_config(config_path.to_str().unwrap());
            }
            // print!("{:?}", config_path)
            pb.inc(5);
            c.generate(args.verbose, &mut pb).unwrap();
        }
        pb.finish_with_message("msg")
    }
}

fn main() {
    let args = Args::parse();

    if let Commands::Generate(generate) = args.commands {
        for path in &generate.config {
            if let Some(extension) = Path::new(path).extension() {
                if extension != "echo" {
                    println!("Error: Config file extension must be .echo: {}", path);
                    continue;
                }
            } else {
                println!("Error: Invalid path format: {}", path);
                continue;
            }

            // Check if file exists and is regular
            if let Ok(metadata) = fs::metadata(path) {
                if !metadata.is_file() {
                    println!("Error: Path is not a regular file: {}", path);
                    continue;
                }
            } else {
                println!("Error: No such file or directory: {}", path);
                continue;
            }

            // Generate wasm file
            generate_wasm();
        }
    }
}
