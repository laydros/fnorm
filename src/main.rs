use clap::Parser;
use fnorm::{run, AppError, Cli};

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    run(&cli)
}
