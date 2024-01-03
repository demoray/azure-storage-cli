use crate::args;
use azure_storage_blobs::prelude::*;
use clap::Subcommand;
use futures::StreamExt;
use std::num::NonZeroU32;

#[derive(Subcommand)]
pub enum AccountSubCommands {
    Info,
    ListContainers {
        #[clap(long)]
        prefix: Option<String>,
        #[clap(long)]
        include_metadata: bool,
        #[clap(long)]
        include_deleted: bool,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
    },
}

pub async fn account_commands(
    service_client: &BlobServiceClient,
    subcommand: AccountSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        AccountSubCommands::Info => {
            let info = service_client.get_account_information().await?;
            println!("{info:#?}");
        }
        AccountSubCommands::ListContainers {
            prefix,
            include_deleted,
            include_metadata,
            max_results,
        } => {
            let mut builder = service_client
                .list_containers()
                .include_deleted(include_deleted)
                .include_metadata(include_metadata);
            args!(builder, prefix, max_results);
            let mut stream = builder.into_stream();
            while let Some(result) = stream.next().await {
                let result = result?;
                for container in &result.containers {
                    println!("{container:#?}");
                }
            }
        }
    }
    Ok(())
}
