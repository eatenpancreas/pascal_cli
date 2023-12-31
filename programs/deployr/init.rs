use clap::Args;

#[derive(Debug, Args)]
pub struct InitOptions {
    /// Adds user
    #[arg(short, long)]
    github_user: Option<String>,
    #[arg(short, long)]
    github_password: Option<String>,
}


pub(crate) fn init(options: InitOptions) {

}