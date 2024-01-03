use crate::args;
use crate::utils::{parse_key_val, to_properties};
use azure_storage_datalake::clients::DataLakeClient;
use clap::Subcommand;
use futures::StreamExt;
use std::num::NonZeroU32;

#[derive(Subcommand)]
pub enum DatalakeSubCommands {
    ListFileSystems {
        #[clap(long)]
        prefix: Option<String>,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
    },
    FileSystem {
        name: String,

        #[clap(subcommand)]
        subcommand: FileSystemSubCommands,
    },
}

#[derive(Subcommand)]
pub enum FileSystemSubCommands {
    /// Create the specified filesystem
    Create {
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        properties: Option<Vec<(String, String)>>,
    },
    /// Create the specified filesystem
    Delete,
    /// List paths in the specified file system
    ListPaths {
        #[clap(long)]
        recursive: Option<bool>,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
        #[clap(long)]
        upn: Option<bool>,
        #[clap(long)]
        directory: Option<String>,
    },
    /// Perform operations on the specified directory
    Directory {
        directory_name: String,
        #[clap(subcommand)]
        subcommand: DirectorySubCommands,
    },
}

#[derive(Subcommand)]
pub enum DirectorySubCommands {
    Create {
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        properties: Option<Vec<(String, String)>>,
        // TODO: support mode, resource, if_match_condition, and if_modified_since
    },
    // TODO: support recursive, if_match_condition, if_modified_since
    Delete {
        #[clap(long)]
        recursive: bool,
    },
    ListPaths {
        #[clap(long)]
        recursive: Option<bool>,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
        #[clap(long)]
        upn: Option<bool>,
        #[clap(long)]
        directory: Option<String>,
    },
}

pub async fn datalake_commands(
    service_client: &DataLakeClient,
    subcommand: DatalakeSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        DatalakeSubCommands::FileSystem { name, subcommand } => {
            let filesystem = service_client.file_system_client(name);
            match subcommand {
                FileSystemSubCommands::Create { properties } => {
                    let properties = properties.map(to_properties);
                    let mut builder = filesystem.create();
                    args!(builder, properties);
                    let result = builder.await?;
                    println!("{result:#?}");
                }
                FileSystemSubCommands::Delete => {
                    // TODO: add support for if_modified_since
                    let result = filesystem.delete().await?;
                    println!("{result:#?}");
                }
                FileSystemSubCommands::ListPaths {
                    recursive,
                    max_results,
                    upn,
                    directory,
                } => {
                    let mut builder = filesystem.list_paths();
                    args!(builder, recursive, directory, max_results, upn);
                    let mut stream = builder.into_stream();
                    while let Some(result) = stream.next().await {
                        let result = result?;
                        for entry in &result.paths {
                            println!("{entry:#?}");
                        }
                    }
                }
                FileSystemSubCommands::Directory {
                    directory_name,
                    subcommand,
                } => {
                    let directory_client = filesystem.get_directory_client(directory_name);
                    match subcommand {
                        DirectorySubCommands::Create { properties } => {
                            let properties = properties.map(to_properties);
                            let mut builder = directory_client.create();
                            args!(builder, properties);
                            let result = builder.await?;
                            println!("{result:#?}");
                        }
                        DirectorySubCommands::ListPaths {
                            recursive,
                            max_results,
                            upn,
                            directory,
                        } => {
                            let mut builder = directory_client.list_paths();
                            args!(builder, recursive, directory, max_results, upn);
                            let mut stream = builder.into_stream();
                            while let Some(result) = stream.next().await {
                                let result = result?;
                                for entry in &result.paths {
                                    println!("{entry:#?}");
                                }
                            }
                        }
                        DirectorySubCommands::Delete { recursive } => {
                            let result = directory_client.delete(recursive).await?;
                            println!("{result:#?}");
                        }
                    }
                }
            }
        }
        DatalakeSubCommands::ListFileSystems {
            prefix,
            max_results,
        } => {
            let mut builder = service_client.list_file_systems();
            args!(builder, prefix, max_results);
            let mut stream = builder.into_stream();
            while let Some(result) = stream.next().await {
                let result = result?;
                for entry in &result.file_systems {
                    println!("{entry:#?}");
                }
            }
        }
    }
    Ok(())
}
