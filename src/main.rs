#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![deny(clippy::indexing_slicing)]
#![allow(clippy::module_name_repetitions)]

mod account;
mod blob;
#[macro_use]
mod macros;
mod container;
mod datalake;
mod queue;
mod utils;

use self::{
    account::{account_commands, AccountSubCommands},
    container::{container_commands, ContainerSubCommands},
    datalake::{datalake_commands, DatalakeSubCommands},
    queue::{queues_commands, QueuesSubCommands},
};
use anyhow::{ensure, Result};
use azure_core::auth::Secret;
use azure_identity::DefaultAzureCredential;
use azure_storage::prelude::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;
use azure_storage_datalake::prelude::DataLakeClient;
use azure_storage_queues::prelude::QueueServiceClient;
use clap::{Command, CommandFactory, Parser, Subcommand};
use std::sync::Arc;
use tokio::fs::read;

#[derive(Parser)]
#[command(
    author,
    version,
    propagate_version = true,
    disable_help_subcommand = true
)]
struct Args {
    /// storage account name.  Set the environment variable STORAGE_ACCOUNT to set a default
    #[clap(long, env = "STORAGE_ACCOUNT", hide_env_values = true)]
    account: String,

    #[command(subcommand)]
    subcommand: SubCommands,

    #[clap(long)]
    use_default_credentials: bool,

    /// storage account access key.  If not set, authentication will be done via
    /// Azure Entra Id using the `DefaultAzureCredential`
    /// (see https://docs.rs/azure_identity/latest/azure_identity/struct.DefaultAzureCredential.html)
    #[clap(long, env = "STORAGE_ACCESS_KEY", hide_env_values = true)]
    access_key: Option<Secret>,
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
    Queues {
        #[clap(subcommand)]
        subcommand: QueuesSubCommands,
    },
    Datalake {
        #[clap(subcommand)]
        datalake: DatalakeSubCommands,
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
            .to_string()
            .replace("\n          \n\n", "\n")
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

    let storage_credentials = match args.access_key {
        Some(access_key) => StorageCredentials::access_key(&args.account, access_key),
        None => StorageCredentials::token_credential(Arc::new(DefaultAzureCredential::default())),
    };

    match args.subcommand {
        SubCommands::Readme { check } => {
            let mut cmd = Args::command();
            let readme = build_readme(&mut cmd, Vec::new())
                .replace("azure-storage-cli", "azs")
                .replacen(
                    "# azs",
                    &format!("# Azure Storage CLI\n\n{}", env!("CARGO_PKG_DESCRIPTION")),
                    1,
                );
            if check {
                let expected = read("README.md").await?;
                ensure!(readme.as_bytes() == expected, "README.md is out of date");
            } else {
                print!("{readme}");
            }
        }
        SubCommands::Account { subcommand } => {
            let service_client = BlobServiceClient::new(&args.account, storage_credentials);
            account_commands(&service_client, subcommand).await?;
        }
        SubCommands::Container {
            subcommand,
            container_name,
        } => {
            let service_client = BlobServiceClient::new(&args.account, storage_credentials);
            let container_client = service_client.container_client(container_name);
            container_commands(&container_client, subcommand).await?;
        }
        SubCommands::Queues { subcommand } => {
            let service_client = QueueServiceClient::new(&args.account, storage_credentials);
            queues_commands(&service_client, subcommand).await?;
        }
        SubCommands::Datalake { datalake } => {
            let service_client = DataLakeClient::new(&args.account, storage_credentials);
            datalake_commands(&service_client, datalake).await?;
        }
    }

    Ok(())
}
