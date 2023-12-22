use crate::{
    errors::IOError,
    types::{Parser, Result},
};
use composer_primitives::{types::BuildDirectory, Exception, OutputDirectory, SourceFiles};
use echo_library::Composer;
use std::path::PathBuf;

pub(crate) struct Context {
    build_directory: Option<BuildDirectory>,
    pub output_directory: Option<OutputDirectory>,
    source_files: Option<SourceFiles>,
    parser: Box<dyn Parser>,
    quiet: bool,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            build_directory: None,
            output_directory: None,
            source_files: None,
            parser: Box::new(Composer::default()),
            quiet: false,
        }
    }
}

impl Context {
    pub fn new() -> Result<Context> {
        Ok(Context::default())
    }

    pub fn quiet(&mut self) {
        self.quiet = true;
    }

    pub fn init(
        &mut self,
        source: Option<PathBuf>,
        build_directory: Option<PathBuf>,
        output_directory: Option<PathBuf>,
    ) -> Result<()> {
        self.build_directory = Some(
            BuildDirectory::new(build_directory)
                .map_err(|x| Box::new(IOError::Anyhow(x)) as Box<dyn Exception>)?,
        );
        self.source_files = Some(SourceFiles::new(source).unwrap());
        self.output_directory = Some(OutputDirectory::new(output_directory).unwrap());

        Ok(())
    }

    pub fn parse(&self) -> Result<()> {
        self.parser.parse(&self.source_files.as_ref().unwrap())
    }

    pub fn build(&self) {
        let _ = &self.parser.build(
            &self.build_directory.as_ref().unwrap(),
            &self.output_directory.as_ref().unwrap(),
            self.quiet,
        );
    }
}
