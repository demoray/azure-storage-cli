use crate::{
    args,
    utils::{parse_key_val, round_up, to_metadata, to_tags},
};
use azure_core::error::{Error, ErrorKind, Result};
use azure_storage_blobs::prelude::{
    BA512Range, BlobClient, BlobContentDisposition, BlobContentEncoding, BlobContentLanguage,
    BlobContentType,
};
use log::debug;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt};
use uuid::Uuid;

#[derive(clap::Parser)]
pub struct CreatePageBlob {
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
}
impl CreatePageBlob {
    pub async fn execute(self, blob_client: &BlobClient) -> Result<()> {
        let CreatePageBlob {
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
        } = self;
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
        Ok(())
    }
}
