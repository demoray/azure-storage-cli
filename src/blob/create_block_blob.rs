use crate::{
    args,
    utils::{parse_key_val, to_metadata, to_tags},
};
use azure_core::{
    error::{Error, ErrorKind, Result},
    request_options::IfTags,
    tokio::fs::FileStreamBuilder,
};
use azure_storage_blobs::prelude::{
    AccessTier, BlobBlockType, BlobClient, BlobContentDisposition, BlobContentEncoding,
    BlobContentLanguage, BlobContentType, BlockList,
};
use std::path::PathBuf;
use tokio::fs::File;
use tracing::debug;
use uuid::Uuid;

/// Create a "block blob" with the contents of the specified file.
#[derive(clap::Parser)]
pub struct CreateBlockBlob {
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
}
impl CreateBlockBlob {
    pub async fn execute(self, blob_client: &BlobClient) -> Result<()> {
        let CreateBlockBlob {
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
        } = self;
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
        Ok(())
    }
}
