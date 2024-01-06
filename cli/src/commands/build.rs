use super::*;
/// Compile and build program command.
#[derive(StructOpt, Debug)]
pub struct Build {
    #[structopt(
        short,
        long,
        help = "Optional path for the build directory",
        parse(from_os_str)
    )]
    pub build_directory: Option<PathBuf>,

    #[structopt(
        short,
        long,
        help = "Optional path to output workflow wasm",
        parse(from_os_str)
    )]
    pub output: Option<PathBuf>,

    pub source: Option<PathBuf>,
}

impl Execute<Context> for Build {
    type Input = ();
    type Output = ();

    fn execute(self, mut context: Context) -> Result<Self::Output> {
        let start = Instant::now();
        context.init(self.source, self.build_directory, self.output)?;
        context.parse()?;
        context.build()?;
        let end = Instant::now();
        let duration = end.duration_since(start);
        if context.quiet{
            println!(
                "   \x1B[32m\x1b[1mBuild Finished\x1b[0m: Workflow created in \x1B[34m\x1b[1m'{:.2?}' \x1b[0m", duration 
            );
        }
        
        Ok(())
    }
}
