use clap::Parser;
use fnorm::{run, Cli, RunError};

fn main() -> Result<(), RunError> {
    let cli = Cli::parse();
    run(&cli)
}
