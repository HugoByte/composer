use clap::{Parser, Arg, Command, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Hugobyte AI Labs", version = "1.0.0", about = "The echo-cli  is a CLI tool used to generate the wasm binary files", long_about = None)]
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
#[command(author, version = "1.0.0", about = "generate the wasm file from the given config file", long_about = None)]
struct Test {
    #[clap(short , long, value_parser)]
    config: Vec<String>
}

fn generate_wasm(){

}

fn main(){
    let args = Args::parse();
   
    println!("{:?}", args.commands);
}