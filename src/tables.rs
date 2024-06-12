use crate::args;
use azure_data_tables::{clients::TableServiceClient, Filter, IfMatchCondition, Select};
use clap::Subcommand;
use futures::StreamExt;
use serde::ser::{SerializeSeq, Serializer};
use serde_json::{ser::PrettyFormatter, Value};
use std::{collections::HashMap, io::stdout, path::PathBuf};

#[derive(Subcommand)]
pub enum TableSubCommands {
    /// List available tables
    ///
    /// The output of this command is serialized as JSON
    List {
        #[clap(long)]
        filter: Option<String>,
        #[clap(long)]
        select: Option<String>,
        #[clap(long)]
        top: Option<u32>,
    },
    /// Create a new table
    Create {
        /// table name
        table_name: String,
    },
    /// Delete a table
    Delete {
        /// table name
        table_name: String,
    },
    /// Query a table
    ///
    /// The output of this command is serialized as JSON
    Query {
        /// table name
        table_name: String,
        #[clap(long)]
        filter: Option<Filter>,
        #[clap(long)]
        select: Option<Select>,
        #[clap(long)]
        top: Option<u32>,
    },
    /// Get a specific row in the table
    ///
    /// The output of this command is serialized as JSON
    Get {
        /// table name
        table_name: String,
        /// Partition Key
        partition_key: String,
        /// Row Key
        row_key: String,
    },
    InsertOrMerge {
        /// table name
        table_name: String,
        /// Partition Key
        partition_key: String,
        /// Row Key
        row_key: String,
        /// JSON file containing the entity
        json_file: PathBuf,
    },
    InsertOrReplace {
        /// table name
        table_name: String,
        /// Partition Key
        partition_key: String,
        /// Row Key
        row_key: String,
        /// JSON file containing the entity
        json_file: PathBuf,
    },
    DeleteEntity {
        /// table name
        table_name: String,
        /// Partition Key
        partition_key: String,
        /// Row Key
        row_key: String,
        /// `ETag` value
        #[clap(long)]
        if_match_condition: Option<String>,
    },
    UpdateEntity {
        /// table name
        table_name: String,
        /// Partition Key
        partition_key: String,
        /// Row Key
        row_key: String,
        /// JSON file containing the entity
        json_file: PathBuf,
        /// `ETag` value
        #[clap(long)]
        if_match_condition: Option<String>,
    },
    MergeEntity {
        /// table name
        table_name: String,
        /// Partition Key
        partition_key: String,
        /// Row Key
        row_key: String,
        /// JSON file containing the entity
        json_file: PathBuf,
        /// `ETag` value
        #[clap(long)]
        if_match_condition: Option<String>,
    },
}

#[allow(clippy::too_many_lines)]
pub async fn table_commands(
    service_client: &TableServiceClient,
    subcommand: TableSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        TableSubCommands::List {
            filter,
            select,
            top,
        } => {
            let mut builder = service_client.list();
            args!(builder, filter, select, top);
            let mut stream = builder.into_stream();

            let mut ser =
                serde_json::Serializer::with_formatter(std::io::stdout(), PrettyFormatter::new());
            let mut serializer = ser.serialize_seq(None)?;

            while let Some(result) = stream.next().await {
                let result = result?;
                for table in &result.tables {
                    serializer.serialize_element(table)?;
                }
            }
            serializer.end()?;
        }
        TableSubCommands::Create { table_name } => {
            service_client.table_client(&table_name).create().await?;
        }
        TableSubCommands::Delete { table_name } => {
            service_client.table_client(&table_name).create().await?;
        }
        TableSubCommands::Query {
            table_name,
            filter,
            select,
            top,
        } => {
            let mut builder = service_client.table_client(&table_name).query();
            args!(builder, filter, select, top);
            let mut stream = builder.into_stream::<Value>();

            let mut ser =
                serde_json::Serializer::with_formatter(std::io::stdout(), PrettyFormatter::new());
            let mut serializer = ser.serialize_seq(None)?;

            while let Some(result) = stream.next().await {
                let result = result?;
                for entity in &result.entities {
                    serializer.serialize_element(entity)?;
                }
            }
            serializer.end()?;
        }
        TableSubCommands::Get {
            table_name,
            partition_key,
            row_key,
        } => {
            let result = service_client
                .table_client(&table_name)
                .partition_key_client(partition_key)
                .entity_client(row_key)
                .get::<Value>()
                .await?;
            serde_json::to_writer_pretty(stdout(), &result.entity)?;
        }
        TableSubCommands::InsertOrMerge {
            table_name,
            partition_key,
            row_key,
            json_file,
        } => {
            let entity: HashMap<String, Value> =
                serde_json::from_reader(std::fs::File::open(json_file)?)?;
            service_client
                .table_client(&table_name)
                .partition_key_client(partition_key)
                .entity_client(row_key)
                .insert_or_merge(&entity)?
                .await?;
        }
        TableSubCommands::InsertOrReplace {
            table_name,
            partition_key,
            row_key,
            json_file,
        } => {
            let entity: HashMap<String, Value> =
                serde_json::from_reader(std::fs::File::open(json_file)?)?;
            service_client
                .table_client(&table_name)
                .partition_key_client(partition_key)
                .entity_client(row_key)
                .insert_or_replace(&entity)?
                .await?;
        }
        TableSubCommands::DeleteEntity {
            table_name,
            partition_key,
            row_key,
            if_match_condition,
        } => {
            let if_match_condition = if_match_condition
                .map_or(IfMatchCondition::Any, |s| IfMatchCondition::Etag(s.into()));
            service_client
                .table_client(&table_name)
                .partition_key_client(partition_key)
                .entity_client(row_key)
                .delete()
                .if_match(if_match_condition)
                .await?;
        }
        TableSubCommands::UpdateEntity {
            table_name,
            partition_key,
            row_key,
            json_file,
            if_match_condition,
        } => {
            let entity: HashMap<String, Value> =
                serde_json::from_reader(std::fs::File::open(json_file)?)?;

            let if_match_condition = if_match_condition
                .map_or(IfMatchCondition::Any, |s| IfMatchCondition::Etag(s.into()));

            service_client
                .table_client(&table_name)
                .partition_key_client(partition_key)
                .entity_client(row_key)
                .update(entity, if_match_condition)?
                .await?;
        }
        TableSubCommands::MergeEntity {
            table_name,
            partition_key,
            row_key,
            json_file,
            if_match_condition,
        } => {
            let entity: HashMap<String, Value> =
                serde_json::from_reader(std::fs::File::open(json_file)?)?;

            let if_match_condition = if_match_condition
                .map_or(IfMatchCondition::Any, |s| IfMatchCondition::Etag(s.into()));

            service_client
                .table_client(&table_name)
                .partition_key_client(partition_key)
                .entity_client(row_key)
                .merge(entity, if_match_condition)?
                .await?;
        }
    }
    Ok(())
}
