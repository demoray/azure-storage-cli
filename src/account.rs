use crate::{args, output_stream_entries, output_stream_entries_debug};
use azure_storage_blobs::prelude::BlobServiceClient;
use clap::Subcommand;
use std::num::NonZeroU32;

#[derive(Subcommand)]
pub enum AccountSubCommands {
    /// Get information about the storage account
    Info,
    /// List the storage containers in the account
    ///
    /// The output of this command is serialized to JSON unless the `show_details` flag is set
    ListContainers {
        #[clap(long)]
        prefix: Option<String>,
        #[clap(long)]
        include_metadata: bool,
        #[clap(long)]
        include_deleted: bool,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
        #[clap(long)]
        show_details: bool,
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
            show_details,
        } => {
            let mut builder = service_client
                .list_containers()
                .include_deleted(include_deleted)
                .include_metadata(include_metadata);
            args!(builder, prefix, max_results);
            if show_details {
                output_stream_entries_debug!(builder.into_stream(), containers);
            } else {
                output_stream_entries!(builder.into_stream(), containers, name);
            }
        }
    }
    Ok(())
}
