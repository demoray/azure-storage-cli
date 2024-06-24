mod create_block_blob;
mod create_page_blob;
mod generate_sas;
mod get;

use crate::{
    args,
    blob::create_block_blob::CreateBlockBlob,
    blob::create_page_blob::CreatePageBlob,
    blob::generate_sas::GenerateSas,
    blob::get::Get,
    utils::{parse_key_val, parse_time, to_metadata, to_tags, TimeFormat},
};
use azure_core::request_options::{IfModifiedSinceCondition, IfTags, LeaseId};
use azure_storage_blobs::prelude::{
    AccessTier, BlobClient, BlobContentDisposition, BlobContentEncoding, BlobContentLanguage,
    BlobContentType, BlobVersioning, DeleteSnapshotsMethod, RehydratePriority, Snapshot, VersionId,
};
use clap::Subcommand;
use log::debug;
use std::path::PathBuf;
use tokio::fs::read;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum BlobSubCommands {
    /// Get the contents of a blob
    Get(Get),
    /// Get properties of a blob
    GetProperties {
        // #[clap(long)]
        // snapshot_id: Option<String>,
        // #[clap(long)]
        // version_id: Option<String>,
        // #[clap(long)]
        // if_modified_since: Option<IfModifiedSinceCondition>,
        // #[clap(long)]
        // if_match: Option<IfMatchCondition>,
        #[clap(long)]
        lease_id: Option<LeaseId>,
        #[clap(long)]
        if_tags: Option<IfTags>,
    },
    /// Delete a blob
    Delete {
        #[clap(long)]
        lease_id: Option<LeaseId>,
        #[clap(long)]
        if_tags: Option<IfTags>,
        #[clap(long)]
        delete_snapshots_method: Option<DeleteSnapshotsMethod>,
    },
    /// Delete the blob at a specific version
    DeleteVersionId {
        version_id: VersionId,
        #[clap(long)]
        lease_id: Option<LeaseId>,
        #[clap(long)]
        permanent: bool,
    },
    /// Delete the blob at a specific version
    DeleteSnapsot {
        snapshot: Snapshot,
        #[clap(long)]
        lease_id: Option<LeaseId>,
        #[clap(long)]
        permanent: bool,
    },
    /// Create a new "append blob" with the contents of the specified file.
    PutAppendBlob {
        #[clap(long)]
        content_type: Option<BlobContentType>,
        #[clap(long)]
        content_encoding: Option<BlobContentEncoding>,
        #[clap(long)]
        content_language: Option<BlobContentLanguage>,
        #[clap(long)]
        content_disposition: Option<BlobContentDisposition>,
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        tags: Option<Vec<(String, String)>>,
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        metadata: Option<Vec<(String, String)>>,
    },
    /// Append the contents of the specified file to an existing "append blob" blob.
    AppendBlock {
        path: PathBuf,
        #[clap(long)]
        condition_max_size: Option<u64>,
        #[clap(long)]
        condition_append_position: Option<u64>,
        #[clap(long)]
        if_tags: Option<IfTags>,
        #[clap(long)]
        lease_id: Option<Uuid>,
        // #[clap(long)]
        // if_modified_since: Option<IfModifiedSinceCondition>,
        // #[clap(long)]
        // if_match: Option<IfMatchCondition>,
    },
    /// Create a "block blob" with the contents of the specified file.
    CreateBlockBlob(CreateBlockBlob),
    /// Create a "page blob" with the contents of the specified file.
    CreatePageBlob(CreatePageBlob),
    /// Generate a SAS URL for the Blob using a User Deligation Key
    GenerateSas(GenerateSas),
    /// Get the tags on the blob
    GetTags {
        if_tags: Option<IfTags>,
        lease_id: Option<LeaseId>,
        snapshot: Option<Snapshot>,
        version_id: Option<VersionId>,
    },
    /// Set the tags on the blob
    SetTags {
        if_tags: Option<IfTags>,
        lease_id: Option<LeaseId>,
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        tags: Option<Vec<(String, String)>>,
    },
    /// Create a snapshot of the blob
    Snapshot {
        // TODO: if_match
        #[clap(long)]
        unmodified_since: Option<String>,
        #[clap(long)]
        modified_since: Option<String>,
        #[clap(long, default_value = "TimeFormat::Offset")]
        time_format: TimeFormat,
        #[clap(long)]
        if_tags: Option<IfTags>,
        #[clap(long)]
        lease_id: Option<LeaseId>,
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        metadata: Option<Vec<(String, String)>>,
    },
    /// Set the access tier on the blob
    SetBlobTier {
        #[clap(long)]
        tier: AccessTier,
        #[clap(long)]
        rehydrate_priority: Option<RehydratePriority>,
        #[clap(long)]
        if_tags: Option<IfTags>,
        #[clap(long)]
        snapshot: Option<Snapshot>,
        #[clap(long)]
        version_id: Option<VersionId>,
    },
}

#[allow(clippy::too_many_lines)]
pub async fn blob_commands(
    blob_client: &BlobClient,
    subcommand: BlobSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        BlobSubCommands::Get(get) => {
            get.execute(blob_client).await?;
        }
        BlobSubCommands::GetProperties { lease_id, if_tags } => {
            let mut builder = blob_client.get_properties();
            args!(builder, lease_id, if_tags);
            let response = builder.await?;
            println!("{response:#?}");
        }
        BlobSubCommands::Delete {
            lease_id,
            if_tags,
            delete_snapshots_method,
        } => {
            let mut builder = blob_client.delete();
            args!(builder, lease_id, if_tags, delete_snapshots_method);
            let response = builder.await?;
            debug!("{response:#?}");
        }
        BlobSubCommands::PutAppendBlob {
            content_type,
            content_encoding,
            content_language,
            content_disposition,
            tags,
            metadata,
        } => {
            let tags = tags.map(to_tags);
            let metadata = metadata.map(to_metadata);
            let mut builder = blob_client.put_append_blob();
            args!(
                builder,
                content_type,
                content_encoding,
                content_language,
                content_disposition,
                tags,
                metadata
            );
            let response = builder.await?;
            debug!("{response:#?}");
        }
        BlobSubCommands::AppendBlock {
            path,
            condition_max_size,
            condition_append_position,
            if_tags,
            lease_id,
        } => {
            let bytes = read(path).await?;
            let mut builder = blob_client.append_block(bytes);
            args!(
                builder,
                condition_max_size,
                condition_append_position,
                if_tags,
                lease_id
            );
            let response = builder.await?;
            debug!("{response:#?}");
        }
        BlobSubCommands::CreateBlockBlob(create) => {
            create.execute(blob_client).await?;
        }
        BlobSubCommands::CreatePageBlob(create) => {
            create.execute(blob_client).await?;
        }
        BlobSubCommands::GenerateSas(generate) => {
            generate.execute(blob_client).await?;
        }
        BlobSubCommands::GetTags {
            if_tags,
            lease_id,
            snapshot,
            version_id,
        } => {
            let mut builder = blob_client.get_tags();
            let blob_versioning = snapshot
                .map(BlobVersioning::Snapshot)
                .or(version_id.map(BlobVersioning::VersionId));
            args!(builder, if_tags, lease_id, blob_versioning);
            let response = builder.await?;
            println!("{response:#?}");
        }
        BlobSubCommands::SetTags {
            if_tags,
            lease_id,
            tags,
        } => {
            let tags = tags.map(to_tags).unwrap_or_default();
            let mut builder = blob_client.set_tags(tags);
            args!(builder, if_tags, lease_id);
            let response = builder.await?;
            println!("{response:#?}");
        }
        BlobSubCommands::Snapshot {
            unmodified_since,
            modified_since,
            time_format,
            if_tags,
            lease_id,
            metadata,
        } => {
            let modified_since = modified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let unmodified_since = unmodified_since
                .map(|s| parse_time(&s, time_format))
                .transpose()?;
            let if_modified_since = modified_since
                .map(IfModifiedSinceCondition::Modified)
                .or_else(|| unmodified_since.map(IfModifiedSinceCondition::Unmodified));
            let metadata = metadata.map(to_metadata);

            let mut builder = blob_client.snapshot();
            args!(builder, if_tags, if_modified_since, lease_id, metadata);
            let response = builder.await?;
            println!("{response:#?}");
        }
        BlobSubCommands::DeleteSnapsot {
            snapshot,
            lease_id,
            permanent,
        } => {
            let mut builder = blob_client.delete_snapshot(snapshot);
            let permanent = Some(permanent);
            args!(builder, lease_id, permanent);
            let response = builder.await?;
            println!("{response:#?}");
        }
        BlobSubCommands::DeleteVersionId {
            version_id,
            lease_id,
            permanent,
        } => {
            let mut builder = blob_client.delete_version_id(version_id);
            let permanent = Some(permanent);
            args!(builder, lease_id, permanent);
            let response = builder.await?;
            println!("{response:#?}");
        }
        BlobSubCommands::SetBlobTier {
            tier,
            rehydrate_priority,
            if_tags,
            snapshot,
            version_id,
        } => {
            let blob_versioning = snapshot
                .map(BlobVersioning::Snapshot)
                .or(version_id.map(BlobVersioning::VersionId));

            let mut builder = blob_client.set_blob_tier(tier);
            args!(builder, rehydrate_priority, if_tags, blob_versioning);
            let response = builder.await?;
            println!("{response:#?}");
        }
    }
    Ok(())
}
