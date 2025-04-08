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
mod tables;
mod utils;

use self::{
    account::{account_commands, AccountSubCommands},
    container::{container_commands, ContainerSubCommands},
    datalake::{datalake_commands, DatalakeSubCommands},
    queue::{queues_commands, QueuesSubCommands},
    tables::{table_commands, TableSubCommands},
};
use anyhow::{anyhow, Result};
use azure_data_tables::clients::TableServiceClient;
use azure_identity::create_default_credential;
use azure_storage::prelude::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;
use azure_storage_datalake::prelude::DataLakeClient;
use azure_storage_queues::prelude::QueueServiceClient;
use clap::{Command, CommandFactory, Parser, Subcommand};
use std::{cmp::min, fmt::Write as _, io::stderr};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    author,
    version,
    propagate_version = true,
    disable_help_subcommand = true
)]
struct Args {
    /// Storage account name
    #[clap(env = "STORAGE_ACCOUNT", hide_env_values = true)]
    account: String,

    #[command(subcommand)]
    subcommand: SubCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
enum SubCommands {
    /// Interact with the storage account
    Account {
        #[clap(subcommand)]
        subcommand: AccountSubCommands,
    },
    /// Interact with storage containers (and blobs)
    Container {
        #[clap(subcommand)]
        subcommand: ContainerSubCommands,

        /// container name
        container_name: String,
    },
    /// Interact with storage queues
    Queues {
        #[clap(subcommand)]
        subcommand: QueuesSubCommands,
    },
    /// Interact with storage datalakes
    Datalake {
        #[clap(subcommand)]
        subcommand: DatalakeSubCommands,
    },
    /// Interact with data tables
    Tables {
        #[clap(subcommand)]
        subcommand: TableSubCommands,
    },
    #[command(hide = true)]
    Readme,
}

fn build_readme(cmd: &mut Command, mut names: Vec<String>) -> String {
    let mut readme = String::new();
    let base_name = cmd.get_name().to_owned();

    names.push(base_name);

    // add positions to the display name if there are any
    for positional in cmd.get_positionals() {
        names.push(format!("<{}>", positional.get_id().as_str().to_uppercase()));
    }

    let name = names.join(" ");

    // once we're at 6 levels of nesting, don't nest anymore.  This is the max
    // that shows up on crates.io and GitHub.
    for _ in 0..(min(names.len(), 6)) {
        readme.push('#');
    }

    let _ = write!(readme, " {name}\n\n```\n{}\n```\n", cmd.render_long_help());

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
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()
                .map_err(|e| anyhow!("invalid env filter: {}", e.to_string()))?,
        )
        .with_writer(stderr)
        .init();

    let Args {
        account,
        subcommand,
    } = Args::parse();

    let storage_credentials = StorageCredentials::token_credential(create_default_credential()?);

    match subcommand {
        SubCommands::Readme => {
            let mut cmd = Args::command();
            let readme = build_readme(&mut cmd, Vec::new())
                .replace("azure-storage-cli", "azs")
                .replace("azs.exe", "azs")
                .replacen(
                    "# azs",
                    &format!("# Azure Storage CLI\n\n{}", env!("CARGO_PKG_DESCRIPTION")),
                    1,
                )
                .lines()
                .map(str::trim_end)
                .collect::<Vec<_>>()
                .join("\n")
                .replace("\n\n\n", "\n");
            print!("{readme}");
        }
        SubCommands::Account { subcommand } => {
            let service_client = BlobServiceClient::new(&account, storage_credentials);
            account_commands(&service_client, subcommand).await?;
        }
        SubCommands::Container {
            subcommand,
            container_name,
        } => {
            let service_client = BlobServiceClient::new(&account, storage_credentials);
            let container_client = service_client.container_client(container_name);
            container_commands(&container_client, subcommand).await?;
        }
        SubCommands::Queues { subcommand } => {
            let service_client = QueueServiceClient::new(&account, storage_credentials);
            queues_commands(&service_client, subcommand).await?;
        }
        SubCommands::Datalake { subcommand } => {
            let service_client = DataLakeClient::new(&account, storage_credentials);
            datalake_commands(&service_client, subcommand).await?;
        }
        SubCommands::Tables { subcommand } => {
            let table_client = TableServiceClient::new(&account, storage_credentials);
            table_commands(&table_client, subcommand).await?;
        }
    }

    Ok(())
}
