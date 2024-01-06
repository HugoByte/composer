use super::*;
/// Compile the config file.
#[derive(StructOpt, Debug)]
pub struct Validate {
    pub source: Option<PathBuf>,
}

impl Execute<Context> for Validate {
    type Input = ();
    type Output = ();

    fn execute(self, mut context: Context) -> Result<Self::Output> {
        let start = Instant::now();
        context.init(self.source, None, None)?;
        context.parse()?;
        let end = Instant::now();

        let duration = end.duration_since(start);
        println!(
            "   \x1B[32m\x1b[1mValidated\x1b[0m: Workflow package are compiled in \x1B[34m\x1b[1m'{:.2?}' \x1b[0m", duration 
        );
        Ok(())
    }
}
