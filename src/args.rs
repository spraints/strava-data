use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Verify that the archive can be parsed.
    Verify(VerifyArgs),
}

#[derive(Args)]
pub struct VerifyArgs {
    /// The directory that was downloaded.
    pub dir: String,
}
