use crate::commands::Commands;
use clap::Parser;

/// Cli Arguments entry point - includes global parameters and subcommands
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Composer",
    long_about = "Composer is a cli tool that empower streamlined cross-platform workflow creation, effortlessly translating configurable files into efficient WebAssembly (Wasm) format for enhanced development and operational efficiency."
)]
#[command(disable_version_flag = true)]
pub struct Cli {
    #[arg(
        short,
        global = true,
        help = "Print additional information for debugging"
    )]
    pub debug: bool,

    #[arg(short, global = true, help = "Suppress CLI output")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,

    /// Print version
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (),
}

impl Cli {
    pub fn quiet(&self) -> bool {
        self.quiet
    }
}
