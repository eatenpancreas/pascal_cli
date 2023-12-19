mod programs;
#[cfg(test)]
mod tests;

use clap::{Parser};
use crate::programs::{Program, run_program};

/// Pan is a all-around CLI made by pascal, for pascal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    /// Available programs
    program: Program,
}

fn main() {
    let cli = Cli::parse();
    run_program(cli.program);
}
