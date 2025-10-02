use clap::Parser;
use fnorm::{Cli, RunError, run};

fn main() -> Result<(), RunError> {
    let cli = Cli::parse();
    run(&cli)
}
