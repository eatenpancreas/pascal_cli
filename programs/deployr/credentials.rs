use std::io::{BufWriter, Write};
use clap::{Args, Subcommand};
use pan_lib::{get_local_file, write_local_file};

#[derive(Subcommand, Debug)]
pub enum Credentials {
    /// Sets credentials
    Set {
        /// Sets the username for GitHub
        #[arg(long)] 
        github_user: Option<String>,
        /// Sets the password for GitHub
        #[arg(long)] 
        github_pass: Option<String>,
        /// Sets the username for Dockerhub
        #[arg(long)] 
        dockerhub_user: Option<String>,
        /// Sets the password for Dockerhub
        #[arg(long)] 
        dockerhub_pass: Option<String>,
        /// Sets the username for Linode
        #[arg(long)] 
        linode_user: Option<String>,
        /// Sets the password for Linode
        #[arg(long)] 
        linode_pass: Option<String>,
        #[arg(long)]
        cloudflare_auth: Option<String>,
        #[arg(long)]
        cloudflare_expires_on: Option<String>,
    },
    /// Verifies if all credentials are valid
    Verify {
        
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonCredentials {
    dockerhub: JsonCredential,
    linode: JsonCredential,
    github: JsonCredential,
    cloudflare_token: JsonAuthToken
}

impl JsonCredentials {
    fn import() -> serde_json::error::Result<Self> {
        serde_json::from_reader(
            get_local_file("credentials.json"))
    }
    
    fn export(self) {
        let mut writer = BufWriter::new(write_local_file("credentials.json"));
        serde_json::to_writer(&mut writer, &self).unwrap();
        writer.flush().unwrap();
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonCredential {
    username: String,
    password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonAuthToken {
    auth: String,
    expires_on: String,
}

pub fn credentials(credentials: Credentials) {
    match credentials {
        Credentials::Set { 
            github_user, github_pass, 
            dockerhub_user, dockerhub_pass,
            linode_user, linode_pass,
            cloudflare_auth, cloudflare_expires_on
        } => {
            let mut current_creds = JsonCredentials::import().unwrap();
            github_user.set_if_some(&mut current_creds.github.username);
            github_pass.set_if_some(&mut current_creds.github.password);
            dockerhub_user.set_if_some(&mut current_creds.dockerhub.password);
            dockerhub_pass.set_if_some(&mut current_creds.dockerhub.password);
            linode_user.set_if_some(&mut current_creds.linode.password);
            linode_pass.set_if_some(&mut current_creds.linode.password);
            cloudflare_auth.set_if_some(&mut current_creds.cloudflare_token.auth);
            cloudflare_expires_on.set_if_some(&mut current_creds.cloudflare_token.expires_on);
            current_creds.export();
        }
        Credentials::Verify { } => verify()
    }
}

pub fn verify() {
    let mut current_creds = JsonCredentials::import().unwrap();
    
}



pub trait SetIfSomeExt<T> {
    fn set_if_some(self, _: &mut T);
}

impl<T> SetIfSomeExt<T> for Option<T> {
    fn set_if_some(self, affected: &mut T) {
        if let Some(t) = self { *affected = t }
    }
}