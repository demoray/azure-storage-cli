use crate::{
    args,
    utils::{parse_time, Protocol, TimeFormat},
};
use azure_core::error::Result;
use azure_storage::shared_access_signature::{service_sas::BlobSasPermissions, SasProtocol};
use azure_storage_blobs::prelude::BlobClient;

#[allow(clippy::struct_excessive_bools)]
#[derive(clap::Parser)]
pub struct GenerateSas {
    /// Expiration
    expiry: String,
    /// Start time
    #[clap(long)]
    start: Option<String>,
    /// Format used for the start and expiry times
    #[clap(long, value_enum, default_value_t = TimeFormat::Offset)]
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
}

impl GenerateSas {
    pub async fn execute(self, blob_client: &BlobClient) -> Result<()> {
        let GenerateSas {
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
        } = self;
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
        let mut builder = blob_client
            .shared_access_signature(permissions, expiry)
            .await?;
        let protocol = protocol.map(|p| match p {
            Protocol::Https => SasProtocol::Https,
            Protocol::HttpHttps => SasProtocol::HttpHttps,
        });

        args!(builder, ip, identifier, protocol, start);

        let url = blob_client.generate_signed_blob_url(&builder)?;
        println!("{url}");
        Ok(())
    }
}
