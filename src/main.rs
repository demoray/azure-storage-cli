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
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
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
    Account {
        #[clap(subcommand)]
        subcommand: AccountSubCommands,
    },
    Container {
        #[clap(subcommand)]
        subcommand: ContainerSubCommands,

        /// container name
        container_name: String,
    },
    Blob {
        #[clap(subcommand)]
        subcommand: BlobSubCommands,
        /// container name
        container_name: String,
        /// blob name
        blob_name: String,
    },
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let storage_credentials = StorageCredentials::access_key(&args.account, &args.access_key);
    let service_client = BlobServiceClient::new(&args.account, storage_credentials);

    match args.subcommand {
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
