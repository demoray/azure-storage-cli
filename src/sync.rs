use crate::utils::to_globset;
use anyhow::Result;
use async_channel::{bounded, Receiver, Sender};
use azure_storage::{ConsistencyCRC64, ConsistencyMD5};
use azure_storage_blobs::{
    container::operations::BlobItem,
    prelude::{BlobClient, ContainerClient},
};
use clap::ValueEnum;
use futures::future::try_join_all;
use futures::StreamExt;
use std::{collections::BTreeMap, num::NonZeroUsize, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SyncMode {
    /// Syncs the directory to the container
    Up,
    /// Syncs the container to the directory
    Down,
}

struct SyncInfo {
    length: u64,
    consistency: Option<Consistency>,
}

enum Consistency {
    Md5(ConsistencyMD5),
    Crc64(ConsistencyCRC64),
}

pub struct TransferClient {
    blob_client: BlobClient,
    local_path: PathBuf,
}

pub(crate) async fn sync_container(
    container_client: ContainerClient,
    mode: SyncMode,
    local_path: PathBuf,
    delete_destination: bool,
    concurrency: Option<NonZeroUsize>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
) -> Result<()> {
    let include = include.map(to_globset).transpose()?;
    let exclude = exclude.map(to_globset).transpose()?;

    let mut remote_paths = BTreeMap::new();
    let mut local_paths = BTreeMap::new();

    let mut stream = container_client.list_blobs().into_stream();
    while let Some(entry) = stream.next().await {
        let entry = entry?;
        for blob in entry.blobs.items {
            let BlobItem::Blob(blob) = blob else {
                continue;
            };

            let consistency = blob
                .properties
                .content_md5
                .map(Consistency::Md5)
                .or_else(|| blob.properties.content_crc64.map(Consistency::Crc64));

            let path = PathBuf::from(blob.name);
            remote_paths.insert(
                path,
                SyncInfo {
                    length: blob.properties.content_length,
                    consistency,
                },
            );
        }
    }

    for entry in WalkDir::new(&local_path) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        };
        let path = entry.path().strip_prefix(&local_path)?.to_owned();
        local_paths.insert(
            path,
            SyncInfo {
                length: entry.metadata()?.len(),
                consistency: None,
            },
        );
    }

    // we'll queue up to 100 files at a time, but that's not how many we'll
    // process at a time.  that's handled elsewhere
    let (tx, rx) = bounded::<TransferClient>(100);

    let transfers = start_transfers(concurrency, rx);

    Ok(())
}

async fn start_transfers(
    concurrency: Option<NonZeroUsize>,
    rx: Receiver<TransferClient>,
) -> anyhow::Result<()> {
    let uploaders: Vec<_> = (0..usize::max(1, 100))
        .map(|_| start_transfer(rx.clone()))
        .collect();

    try_join_all(uploaders).await?;

    Ok(())
}

async fn start_transfer(rx: Receiver<TransferClient>) -> anyhow::Result<()> {
    while let Ok(transfer) = rx.recv().await {}
    Ok(())
}
