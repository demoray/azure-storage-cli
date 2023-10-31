mod account;
mod blob;
#[macro_use]
mod macros;
mod container;
mod utils;

use crate::{
    account::{account_commands, AccountSubCommands},
    blob::{blob_commands, BlobSubCommands},
    container::{container_commands, ContainerSubCommands},
};
use anyhow::{ensure, Result};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use clap::{Command, CommandFactory, Parser, Subcommand};
use std::env::remove_var;
use tokio::fs::read;

#[derive(Parser)]
#[command(
    author,
    version,
    propagate_version = true,
    disable_help_subcommand = true
)]
struct Args {
    #[command(subcommand)]
    subcommand: SubCommands,

    /// storage account name
    #[clap(env = "STORAGE_ACCOUNT")]
    account: String,
    /// storage account access key
    #[clap(env = "STORAGE_ACCESS_KEY")]
    access_key: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
enum SubCommands {
    /// Interact with the storage account
    Account {
        #[clap(subcommand)]
        subcommand: AccountSubCommands,
    },
    /// Interact with storage containers
    Container {
        #[clap(subcommand)]
        subcommand: ContainerSubCommands,

        /// container name
        container_name: String,
    },
    /// Interact with a blob within a storage container
    Blob {
        #[clap(subcommand)]
        subcommand: BlobSubCommands,
        /// container name
        container_name: String,
        /// blob name
        blob_name: String,
    },
    #[command(hide = true)]
    Readme {
        #[clap(long)]
        check: bool,
    },
}

fn build_readme(cmd: &mut Command, mut names: Vec<String>) -> String {
    let mut readme = String::new();
    let base_name = cmd.get_name().to_owned();

    names.push(base_name);

    let name = names.join(" ");

    for _ in 0..names.len() {
        readme.push('#');
    }
    readme.push_str(&format!(
        " {name}\n\n```\n{}\n```\n",
        cmd.render_long_help()
    ));

    for cmd in cmd.get_subcommands_mut() {
        if cmd.get_name() == "readme" {
            continue;
        }
        readme.push_str(&build_readme(cmd, names.clone()));
    }
    readme
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let storage_credentials = StorageCredentials::access_key(&args.account, &args.access_key);
    let service_client = BlobServiceClient::new(&args.account, storage_credentials);

    match args.subcommand {
        SubCommands::Readme { check } => {
            // in case the variables are set, we don't want to print them in the readme
            for key in ["STORAGE_ACCOUNT", "STORAGE_ACCESS_KEY"] {
                remove_var(key);
            }
            let mut cmd = Args::command();
            let readme = build_readme(&mut cmd, Vec::new());
            if check {
                let expected = read("README.md").await?;
                ensure!(readme.as_bytes() == expected, "README.md is out of date");
            } else {
                print!("{readme}");
            }
        }
        SubCommands::Account { subcommand } => {
            account_commands(&service_client, subcommand).await?;
        }
        SubCommands::Container {
            subcommand,
            container_name,
        } => {
            let container_client = service_client.container_client(container_name);
            container_commands(&container_client, subcommand).await?;
        }
        SubCommands::Blob {
            subcommand,
            container_name,
            blob_name,
        } => {
            let blob_client = service_client
                .container_client(container_name)
                .blob_client(blob_name);
            blob_commands(&blob_client, subcommand).await?;
        }
    }

    Ok(())
}
