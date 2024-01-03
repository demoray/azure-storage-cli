use crate::{
    args,
    utils::{parse_key_val, round_up, to_metadata, to_tags},
};
use azure_core::{
    error::{Error, ErrorKind},
    request_options::{IfTags, LeaseId},
    tokio::fs::FileStreamBuilder,
};
use azure_storage_blobs::prelude::{
    AccessTier, BA512Range, BlobBlockType, BlobClient, BlobContentDisposition, BlobContentEncoding,
    BlobContentLanguage, BlobContentType, BlockList, DeleteSnapshotsMethod,
};
use clap::Subcommand;
use futures::StreamExt;
use log::debug;
use std::{path::PathBuf, pin::Pin};
use tokio::{
    fs::{read, File},
    io::{stdout, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum BlobSubCommands {
    /// Get the contents of a blob
    Get {
        // #[clap(long)]
        // snapshot_id: Option<String>,
        // #[clap(long)]
        // version_id: Option<String>,
        // #[clap(long)]
        // encryption_key: Option<CPKInfo>,
        // #[clap(long)]
        // if_modified_since: Option<IfModifiedSinceCondition>,
        // #[clap(long)]
        // if_match: Option<IfMatchCondition>,
        #[clap(long)]
        lease_id: Option<LeaseId>,
        #[clap(long)]
        chunk_size: Option<u64>,
        #[clap(long)]
        if_tags: Option<IfTags>,

        /// Where should the contents of the file be written (otherwise, written to STDOUT)
        destination: Option<PathBuf>,
    },
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
    CreateBlockBlob {
        path: PathBuf,
        /// Upload the file in blocks of this size
        #[clap(long)]
        upload_block_size: Option<u64>,
        /// How much to buffer in memory while uploading
        #[clap(long)]
        buffer_size: Option<usize>,
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
        #[clap(long)]
        if_tags: Option<IfTags>,
        #[clap(long)]
        lease_id: Option<Uuid>,
        #[clap(long)]
        access_tier: Option<AccessTier>,
    },
    /// Create a "page blob" with the contents of the specified file.
    CreatePageBlob {
        path: PathBuf,
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
        #[clap(long)]
        lease_id: Option<Uuid>,
        #[clap(long)]
        sequence_number: Option<u64>,
        #[clap(long)]
        upload_block_size: Option<usize>,
    },
}

#[allow(clippy::too_many_lines)]
pub async fn blob_commands(
    blob_client: &BlobClient,
    subcommand: BlobSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        BlobSubCommands::Get {
            lease_id,
            chunk_size,
            if_tags,
            destination,
        } => {
            let mut builder = blob_client.get();
            args!(builder, lease_id, chunk_size, if_tags);

            let mut handle: Pin<Box<dyn AsyncWrite>> = if let Some(destination) = destination {
                Box::pin(File::create(destination).await?)
            } else {
                Box::pin(stdout())
            };

            let mut stream = builder.into_stream();
            while let Some(blob_entry) = stream.next().await {
                let mut blob_entry = blob_entry?;
                debug!("{blob_entry:#?}");
                while let Some(chunk) = blob_entry.data.next().await {
                    let chunk = chunk?;
                    handle.write_all(&chunk).await?;
                }
            }
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
        BlobSubCommands::CreateBlockBlob {
            path,
            upload_block_size: block_size,
            buffer_size,
            content_type,
            content_encoding,
            content_language,
            content_disposition,
            tags,
            metadata,
            if_tags,
            lease_id,
            access_tier,
        } => {
            let handle = File::open(path).await?;
            let mut builder = FileStreamBuilder::new(handle);
            args!(builder, buffer_size, block_size);
            let mut handle = builder.build().await?;

            let tags = tags.map(to_tags);
            let metadata = metadata.map(to_metadata);

            if let Some(block_size) = block_size {
                let mut block_list = BlockList::default();

                for offset in (handle.offset..handle.stream_size).step_by(
                    usize::try_from(block_size)
                        .map_err(|e| Error::new(ErrorKind::DataConversion, e))?,
                ) {
                    let block_id = format!("{offset:08X}");
                    let mut builder = blob_client.put_block(block_id.clone(), &handle);
                    args!(builder, lease_id);
                    let response = builder.await?;
                    debug!("{response:#?}");
                    block_list
                        .blocks
                        .push(BlobBlockType::new_uncommitted(block_id));
                    handle.next_block().await?;
                }
                let mut builder = blob_client.put_block_list(block_list);
                args!(
                    builder,
                    if_tags,
                    lease_id,
                    content_type,
                    content_language,
                    content_disposition,
                    content_encoding,
                    access_tier,
                    tags,
                    metadata
                );
                let response = builder.await?;
                debug!("{response:#?}");
            } else {
                let mut builder = blob_client.put_block_blob(handle);

                args!(
                    builder,
                    if_tags,
                    lease_id,
                    content_type,
                    content_language,
                    content_disposition,
                    content_encoding,
                    access_tier,
                    tags,
                    metadata
                );

                let response = builder.await?;
                debug!("{response:#?}");
            }
        }
        BlobSubCommands::CreatePageBlob {
            path,
            content_type,
            content_encoding,
            content_language,
            content_disposition,
            tags,
            metadata,
            lease_id,
            sequence_number,
            upload_block_size,
        } => {
            let tags = tags.map(to_tags);
            let metadata = metadata.map(to_metadata);

            let mut handle = File::open(path).await?;
            let length = handle.metadata().await?.len();

            let rounded_up = round_up(length, 512);
            let mut builder = blob_client.put_page_blob(u128::from(rounded_up));
            args!(
                builder,
                content_type,
                content_encoding,
                content_language,
                content_disposition,
                tags,
                metadata,
                lease_id,
                sequence_number
            );
            let result = builder.await?;
            debug!("{result:#?}");

            let upload_block_size = upload_block_size.unwrap_or(4 * 1024 * 1024);

            for start in (0..=rounded_up).step_by(upload_block_size) {
                let mut take_handle = handle.take(upload_block_size as u64);
                let mut buf = vec![];
                let read_size = take_handle.read_to_end(&mut buf).await?;
                let rounded_up = round_up(read_size as u64, 512);

                buf.resize(
                    usize::try_from(rounded_up)
                        .map_err(|e| Error::new(ErrorKind::DataConversion, e))?,
                    0,
                );
                handle = take_handle.into_inner();

                let ba512_range = BA512Range::new(start, start + rounded_up - 1)?;
                let mut builder = blob_client.put_page(ba512_range, buf);
                args!(builder, lease_id);
                let response = builder.await?;
                debug!("{response:#?}");
            }
        }
    }
    Ok(())
}
