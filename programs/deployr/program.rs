mod verify;
mod init;
mod credentials;

use clap::{Args, Subcommand};

use verify::verify;
use DTool::*;
use crate::credentials::{Credentials, credentials};
use crate::init::{init, InitOptions};

#[derive(Debug, Args)]
pub struct ProgramArgs {
    /// The tool that deployr will use
    #[clap(subcommand)]
    tool: DTool,
}

#[derive(Subcommand, Debug)]
enum DTool {
    /// Sets up project workflow
    Init (InitOptions),
    #[clap(subcommand)]
    Creds (Credentials),
    /// Verifies project
    Verify {
        /// The directory that the template will be put in.
        #[arg(short, long, default_value = ".")]
        dir: String,
    }
}

pub fn run(args: ProgramArgs) {
    match args.tool {
        Verify { dir } => { verify(dir); }
        Init (options) => init(options),
        Creds(creds) => credentials(creds)
    }
}