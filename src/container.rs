use crate::{
    args,
    utils::{parse_key_val, to_metadata},
};
use azure_storage_blobs::prelude::*;
use clap::Subcommand;
use futures::StreamExt;
use std::num::NonZeroU32;
use uuid::Uuid;

#[derive(Subcommand)]
pub(crate) enum ContainerSubCommands {
    Create {
        /// public access level
        #[clap(long)]
        public_access: Option<PublicAccess>,

        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        metadata: Option<Vec<(String, String)>>,
    },
    Delete {
        /// lease id
        #[clap(long)]
        lease_id: Option<Uuid>,
    },
    List {
        /// only include blobs with the specified prefix
        #[clap(long)]
        prefix: Option<String>,
        /// only include blobs with the specified delimiter
        #[clap(long)]
        delimiter: Option<String>,
        /// max results to return
        #[clap(long)]
        max_results: Option<NonZeroU32>,
        #[clap(long)]
        include_snapshots: bool,
        #[clap(long)]
        include_metadata: bool,
        #[clap(long)]
        include_uncommited_blobs: bool,
        #[clap(long)]
        include_copy: bool,
        #[clap(long)]
        include_deleted: bool,
        #[clap(long)]
        include_tags: bool,
        #[clap(long)]
        include_versions: bool,
    },
}

pub(crate) async fn container_commands(
    container_client: &ContainerClient,
    subcommand: ContainerSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        ContainerSubCommands::Create {
            public_access,
            metadata,
        } => {
            let mut builder = container_client.create();
            let metadata = metadata.map(to_metadata);
            args!(builder, metadata, public_access);
            builder.await?;
        }
        ContainerSubCommands::Delete { lease_id } => {
            let mut builder = container_client.delete();
            args!(builder, lease_id);
            builder.await?;
        }
        ContainerSubCommands::List {
            prefix,
            delimiter,
            max_results,
            include_snapshots,
            include_metadata,
            include_uncommited_blobs,
            include_copy,
            include_deleted,
            include_tags,
            include_versions,
        } => {
            let mut builder = container_client
                .list_blobs()
                .include_snapshots(include_snapshots)
                .include_metadata(include_metadata)
                .include_uncommitted_blobs(include_uncommited_blobs)
                .include_copy(include_copy)
                .include_deleted(include_deleted)
                .include_tags(include_tags)
                .include_versions(include_versions);

            args!(builder, prefix, delimiter, max_results);

            let mut blob_stream = builder.into_stream();
            while let Some(blob_entry) = blob_stream.next().await {
                let blob_entry = blob_entry?;
                for blob in blob_entry.blobs.blobs() {
                    println!("{blob:#?}");
                }
            }
        }
    }
    Ok(())
}
