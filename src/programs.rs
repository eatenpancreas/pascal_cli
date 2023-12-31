
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Program {
    /// Hello world!
    Hwd,
    /// Same as regular Ls
    Ls,
    /// Grabs one of the templates from GitHub and initialises it
    Template(cli_template::ProgramArgs),
    /// Deploys a project straight to Linode
    Deployr(cli_deployr::ProgramArgs),
}

use Program::*;

pub fn run_program(program: Program) {
    match program {
        Hwd => cli_hwd::run(),
        Ls => cli_ls::run(),
        Template(args) => cli_template::run(args),
        Deployr(args) => cli_deployr::run(args),
    }
}
