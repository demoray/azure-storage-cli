use crate::{
    args,
    blob::{blob_commands, BlobSubCommands},
    utils::{parse_duration, parse_key_val, parse_time, to_metadata, Protocol, TimeFormat},
};
use azure_core::{
    prelude::LeaseDuration,
    request_options::{Delimiter, IfModifiedSinceCondition, LeaseId, Prefix},
};
use azure_storage::shared_access_signature::{service_sas::BlobSasPermissions, SasProtocol};
use azure_storage_blobs::prelude::{ContainerClient, PublicAccess};
use clap::Subcommand;
use futures::StreamExt;
use log::debug;
use std::num::NonZeroU32;
use uuid::Uuid;

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum ContainerSubCommands {
    /// Create a storage container
    Create {
        /// public access level
        #[clap(long)]
        public_access: Option<PublicAccess>,

        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        metadata: Option<Vec<(String, String)>>,
    },
    /// Get properties for a storage container
    Properties {
        /// lease id
        #[clap(long)]
        lease_id: Option<Uuid>,
    },
    /// Delete a storage container
    Delete {
        /// lease id
        #[clap(long)]
        lease_id: Option<Uuid>,
    },
    /// List blobs in a storage container
    List {
        /// only include blobs with the specified prefix
        #[clap(long)]
        prefix: Option<Prefix>,
        /// only include blobs with the specified delimiter
        #[clap(long)]
        delimiter: Option<Delimiter>,
        /// max results to return
        #[clap(long)]
        max_results: Option<NonZeroU32>,
        #[clap(long)]
        include_snapshots: bool,
        #[clap(long)]
        include_metadata: bool,
        #[clap(long)]
        include_uncommitted_blobs: bool,
        #[clap(long)]
        include_copy: bool,
        #[clap(long)]
        include_deleted: bool,
        #[clap(long)]
        include_tags: bool,
        #[clap(long)]
        include_versions: bool,
    },
    /// Interact with a blob within a storage container
    Blob {
        #[clap(subcommand)]
        subcommand: BlobSubCommands,
        /// blob name
        blob_name: String,
    },
    /// Generate a SAS URL for a storage container
    GenerateSas {
        /// Expiration
        expiry: String,
        /// Start time
        #[clap(long)]
        start: Option<String>,
        /// Format used for the start and expiry times
        #[clap(long, default_value = "TimeFormat::Offset")]
        time_format: TimeFormat,

        #[clap(long)]
        ip: Option<String>,
        #[clap(long)]
        identifier: Option<String>,
        #[clap(long)]
        protocol: Option<Protocol>,

        #[clap(long)]
        read: bool,
        #[clap(long)]
        add: bool,
        #[clap(long)]
        create: bool,
        #[clap(long)]
        write: bool,
        #[clap(long)]
        delete: bool,
        #[clap(long)]
        delete_version: bool,
        #[clap(long)]
        list: bool,
        #[clap(long)]
        tags: bool,
        #[clap(long, name = "move")]
        move_: bool,
        #[clap(long)]
        execute: bool,
        #[clap(long)]
        ownership: bool,
        #[clap(long)]
        permissions: bool,
        #[clap(long)]
        permanent_delete: bool,
    },
    /// Acquire a lease on a storage container
    AcquireLease {
        /// lease duration in seconds (otherwise uses Infinite)
        lease_duration: Option<u8>,
        proposed_lease_id: Option<LeaseId>,
        lease_id: Option<LeaseId>,

        unmodified_since: Option<String>,
        modified_since: Option<String>,
        #[clap(long, default_value = "TimeFormat::Offset")]
        time_format: TimeFormat,
    },
    /// Acquire a lease on a storage container
    BreakLease {
        /// Duration as parsed by <https://docs.rs/duration-string/latest/duration_string/>
        /// Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`
        lease_break_period: Option<String>,
        lease_id: Option<LeaseId>,
        unmodified_since: Option<String>,
        modified_since: Option<String>,
        #[clap(long, default_value = "TimeFormat::Offset")]
        time_format: TimeFormat,
    },
    LeaseRelease {
        lease_id: LeaseId,
        unmodified_since: Option<String>,
        modified_since: Option<String>,
        #[clap(long, default_value = "TimeFormat::Offset")]
        time_format: TimeFormat,
    },
    LeaseRenew {
        lease_id: LeaseId,
        unmodified_since: Option<String>,
        modified_since: Option<String>,
        #[clap(long, default_value = "TimeFormat::Offset")]
        time_format: TimeFormat,
    },
}

#[allow(clippy::too_many_lines)]
pub async fn container_commands(
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
        ContainerSubCommands::Properties { lease_id } => {
            let mut builder = container_client.get_properties();
            args!(builder, lease_id);
            debug!("{:#?}", builder.await?);
        }
        ContainerSubCommands::List {
            prefix,
            delimiter,
            max_results,
            include_snapshots,
            include_metadata,
            include_uncommitted_blobs,
            include_copy,
            include_deleted,
            include_tags,
            include_versions,
        } => {
            let mut builder = container_client
                .list_blobs()
                .include_snapshots(include_snapshots)
                .include_metadata(include_metadata)
                .include_uncommitted_blobs(include_uncommitted_blobs)
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
        ContainerSubCommands::Blob {
            subcommand,
            blob_name,
        } => {
            blob_commands(&container_client.blob_client(blob_name), subcommand).await?;
        }
        ContainerSubCommands::GenerateSas {
            expiry,
            start,
            time_format,
            ip,
            identifier,
            protocol,
            read,
            add,
            create,
            write,
            delete,
            delete_version,
            list,
            tags,
            move_,
            execute,
            ownership,
            permissions,
            permanent_delete,
        } => {
            let expiry = parse_time(&expiry, time_format)?;
            let start = start.map(|s| parse_time(&s, time_format)).transpose()?;

            let permissions = BlobSasPermissions {
                read,
                add,
                create,
                write,
                delete,
                delete_version,
                permanent_delete,
                list,
                tags,
                move_,
                execute,
                ownership,
                permissions,
            };
            let mut builder = container_client
                .shared_access_signature(permissions, expiry)
                .await?;
            let protocol = protocol.map(|p| match p {
                Protocol::Https => SasProtocol::Https,
                Protocol::HttpHttps => SasProtocol::HttpHttps,
            });

            args!(builder, ip, identifier, protocol, start);

            let url = container_client.generate_signed_container_url(&builder)?;
            println!("{url}");
        }
        ContainerSubCommands::AcquireLease {
            lease_duration,
            lease_id,
            proposed_lease_id,
            time_format,
            unmodified_since,
            modified_since,
        } => {
            let lease_duration =
                lease_duration.map_or(LeaseDuration::Infinite, LeaseDuration::Seconds);
            let mut builder = container_client.acquire_lease(lease_duration);

            let modified_since = modified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let unmodified_since = unmodified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let if_modified_since = modified_since
                .map(IfModifiedSinceCondition::Modified)
                .or_else(|| unmodified_since.map(IfModifiedSinceCondition::Unmodified));

            args!(builder, lease_id, proposed_lease_id, if_modified_since);

            let result = builder.await?;
            println!("{result:#?}");
        }
        ContainerSubCommands::BreakLease {
            lease_break_period,
            lease_id,
            unmodified_since,
            modified_since,
            time_format,
        } => {
            let mut builder = container_client.break_lease();
            let lease_break_period = lease_break_period
                .as_deref()
                .map(parse_duration)
                .transpose()?;

            let modified_since = modified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let unmodified_since = unmodified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let if_modified_since = modified_since
                .map(IfModifiedSinceCondition::Modified)
                .or_else(|| unmodified_since.map(IfModifiedSinceCondition::Unmodified));

            args!(builder, lease_id, if_modified_since, lease_break_period);

            let result = builder.await?;
            println!("{result:#?}");
        }
        ContainerSubCommands::LeaseRelease {
            lease_id,
            modified_since,
            unmodified_since,
            time_format,
        } => {
            let mut builder = container_client.container_lease_client(lease_id).release();

            let modified_since = modified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let unmodified_since = unmodified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let if_modified_since = modified_since
                .map(IfModifiedSinceCondition::Modified)
                .or_else(|| unmodified_since.map(IfModifiedSinceCondition::Unmodified));

            args!(builder, if_modified_since);
            let result = builder.await?;
            println!("{result:#?}");
        }
        ContainerSubCommands::LeaseRenew {
            lease_id,
            modified_since,
            unmodified_since,
            time_format,
        } => {
            let mut builder = container_client.container_lease_client(lease_id).renew();

            let modified_since = modified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let unmodified_since = unmodified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let if_modified_since = modified_since
                .map(IfModifiedSinceCondition::Modified)
                .or_else(|| unmodified_since.map(IfModifiedSinceCondition::Unmodified));

            args!(builder, if_modified_since);
            let result = builder.await?;
            println!("{result:#?}");
        }
    }
    Ok(())
}
