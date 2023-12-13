use clap::Parser;

/// The `Build` struct is used to generate a WebAssembly (wasm) file from a given config file.
///
/// Properties:
///
/// * `config`: A vector of strings representing the configuration files.
/// * `output`: An optional string that represents the output file name or path.
#[derive(Parser, Debug, Clone)]
#[command(author,about = "generate the wasm file from the given config file", long_about = None)]
pub struct Build {
    #[clap(short, long, value_parser)]
    pub config: Vec<String>,
    #[clap(short, long, value_parser)]
    pub output: Option<String>,
}
