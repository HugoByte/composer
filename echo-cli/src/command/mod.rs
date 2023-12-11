pub mod build;

pub use build::Build;
use clap::Subcommand;

/// The code is defining an enum called `Commands` with a single variant called `Build`. The
/// `#[derive(Subcommand, Debug)]` attribute is used to automatically generate implementations for the
/// `Subcommand` and `Debug` traits for the `Commands` enum.
#[derive(Subcommand, Debug)]
pub enum Commands {
    Build( Build )
}