use clap::{Parser, Arg, Command, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

use composer::composer::*;

#[derive(Parser, Debug)]
#[command(author, version = "1.0.0", about = "The echo-cli is a CLI tool used to generate the wasm binary files", long_about = None)]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate(Generate),
    Test(Test)
}

#[derive(Parser, Debug)]
#[command(author, version = "1.0.0", about = "generate the wasm file from the given config file", long_about = None)]
struct Generate {
    #[clap(short , long, value_parser)]
    config: Vec<String>,
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version = "1.0.0", about = "Test the flow of the config file", long_about = None)]
struct Test {
    #[clap(short , long, value_parser)]
    config: Vec<String>
}

fn generate_wasm(){

    let mut c = composer::Composer::default();
    let current_path = std::env::current_dir().unwrap(); // Replace with the actual current directory path
    c.add_config("../config/car_market_place.star");

    // println!("===> {:?}", current_path.join);
    c.run(&current_path.join("../"));
}


fn main(){
    let args = Args::parse();

if let Commands::Generate(generate) = args.commands {
    for path in &generate.config {
        if fs::metadata(PathBuf::from(path)).is_ok() {
            // code for generating wasm file 
            generate_wasm()
        } else {
            println!("Error: No such file or directory");
        }
    }

}
// println!("Config file path: {:?}", config_path);
}