use clap::Parser;
use crate::command::Commands;

#[derive(Parser, Debug)]
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