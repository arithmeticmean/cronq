use clap::Parser;
use url::Url;

/// Global options shared across all commands
#[derive(Debug, Parser)]
pub(super) struct GlobalArgs {
    /// Server address (required, e.g., http://localhost:8428)
    #[arg(required = true, long)]
    pub(super) server: String,

    /// Enable verbose debug output
    #[arg(short, long)]
    pub(super) verbose: bool,
}
