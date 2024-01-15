use crate::args;
use azure_core::{
    error::Result,
    request_options::{IfTags, LeaseId},
};
use azure_storage_blobs::prelude::BlobClient;
use futures::StreamExt;
use log::debug;
use std::{path::PathBuf, pin::Pin};
use tokio::{
    fs::File,
    io::{stdout, AsyncWrite, AsyncWriteExt},
};

#[derive(clap::Parser)]
pub struct Get {
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
}

impl Get {
    pub async fn execute(self, blob_client: &BlobClient) -> Result<()> {
        let Get {
            lease_id,
            chunk_size,
            if_tags,
            destination,
        } = self;
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
        Ok(())
    }
}
