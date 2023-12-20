use composer_library::{SourceFiles, OutputDirectory};

use super::{BuildDirectory, Result};

pub trait Parser {
    fn parse(&self, source: &SourceFiles) -> Result<()>;
    fn build(&self, build_directory: &BuildDirectory, output_directory: &OutputDirectory, quiet: bool);
}
