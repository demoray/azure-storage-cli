use crate::{
    args,
    utils::{parse_time, Protocol, TimeFormat},
};
use azure_core::error::Result;
use azure_storage::shared_access_signature::{service_sas::BlobSasPermissions, SasProtocol};
use azure_storage_blobs::prelude::BlobClient;
use clap::Args;
use time::OffsetDateTime;

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

    #[clap(flatten)]
    sas_permissions: SasPermissions,
}

#[derive(Debug, Args)]
#[group(required = true)]
#[allow(clippy::struct_excessive_bools)]
struct SasPermissions {
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
            sas_permissions:
                SasPermissions {
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
                },
        } = self;
        let expiry = parse_time(&expiry, time_format)?;
        let start = start
            .map(|s| parse_time(&s, time_format))
            .transpose()?
            .unwrap_or_else(OffsetDateTime::now_utc);

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

        let user_deligation_key = blob_client
            .container_client()
            .service_client()
            .get_user_deligation_key(start, expiry)
            .await?;

        let mut builder = blob_client
            .user_delegation_shared_access_signature(
                permissions,
                &user_deligation_key.user_deligation_key,
            )
            .await?;
        let protocol = protocol.map(|p| match p {
            Protocol::Https => SasProtocol::Https,
            Protocol::HttpHttps => SasProtocol::HttpHttps,
        });

        args!(builder, ip, identifier, protocol);

        let url = blob_client.generate_signed_blob_url(&builder)?;
        println!("{url}");
        Ok(())
    }
}
