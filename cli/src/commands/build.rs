use clap::Parser;

use super::*;

/// Compile and build program command.
#[derive(Parser, Debug)]
pub struct Build {
    #[arg(
        short,
        long,
        help = "Optional Rust package generation path is created solely in case of build failure"
    )]
    pub build_dir: Option<PathBuf>,

    #[arg(short, long, help = "Optional path to output workflow wasm")]
    pub output: Option<PathBuf>,

    /// The path(relative path or absolute path) of the directory, where the package is located. This is an optional path if you are in the current directory 
    pub source: Option<PathBuf>,
}

impl Execute<Context> for Build {
    type Input = ();
    type Output = ();

    fn execute(self, mut context: Context) -> Result<Self::Output> {
        let start = Instant::now();
        context.init(self.source, self.build_dir, self.output)?;
        context.parse()?;
        context.build()?;
        let end = Instant::now();
        let duration = end.duration_since(start);
        if context.quiet {
            println!(
                "   \x1B[32m\x1b[1mBuild Finished\x1b[0m: Workflow created in \x1B[34m\x1b[1m'{:.2?}' \x1b[0m", duration 
            );
        }

        Ok(())
    }
}
