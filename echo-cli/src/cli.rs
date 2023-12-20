use clap::Parser;
use crate::command::Commands;
/// The `CLI` struct is a command-line interface tool used to generate wasm binary files.
/// 
/// Properties:
/// 
/// * `commands`: The `commands` field is of type `Commands`, which is a subcommand for the CLI tool. It
/// represents the different commands that can be executed by the tool.
/// * `verbose`: A boolean flag that determines whether to display verbose output or not. It is set to
/// false by default.

#[derive(Parser, Debug, Clone)]
#[command(author, version = "0.0.1", about = "The echo-cli is a CLI tool used to generate the wasm binary files", long_about = None)]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,
    #[structopt(
        long = "verbose",
        short,
        global = true,
        default_value = "false",
        help = "Suppress CLI output"
    )]
    pub verbose: bool,
}