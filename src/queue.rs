use crate::args;
use crate::utils::{parse_key_val, to_metadata};
use azure_storage_queues::{PopReceipt, QueueServiceClient};
use clap::Subcommand;
use futures::StreamExt;
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Subcommand)]
pub enum QueueSubCommands {
    GetProperties,
    ListQueues {
        #[clap(long)]
        prefix: Option<String>,
        #[clap(long)]
        include_metadata: bool,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
    },
    /// Create a Storage Queue
    Create {
        /// Name of the queue
        queue_name: String,
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        metadata: Option<Vec<(String, String)>>,
    },
    /// Delete a Storage Queue
    Delete {
        /// Name of the queue
        queue_name: String,
    },
    /// Put a message onto the Storage Queue
    PutMessage {
        /// Name of the queue
        queue_name: String,
        message: String,
        // TTL (in seconds)
        #[clap(long)]
        ttl: Option<u64>,
        // Visibility timeout (in seconds)
        #[clap(long)]
        visibility_timeout: Option<u64>,
    },
    /// Clear all messages on a storage queue
    Clear {
        /// Name of the queue
        queue_name: String,
    },
    /// Get messages from a storage queue
    GetMessages {
        /// Name of the queue
        queue_name: String,
        #[clap(long)]
        number_of_messages: Option<u8>,
        #[clap(long)]
        // Visibility timeout (in seconds)
        visibility_timeout: Option<u64>,
    },
    /// Peek at available messages from a storage queue
    PeekMessages {
        /// Name of the queue
        queue_name: String,
        #[clap(long)]
        number_of_messages: Option<u8>,
    },
    /// Pop a message from a storage queue
    PopMessage {
        /// Name of the queue
        queue_name: String,
        /// Message ID for the message to be deleted (usually from the `GetMessages` response)
        message_id: String,
        /// Pop Receipt the message to be deleted (usually from the `GetMessages` response)
        pop_receipt: String,
    },
}

pub async fn queue_commands(
    service_client: &QueueServiceClient,
    subcommand: QueueSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        QueueSubCommands::GetProperties => {
            let info = service_client.get_queue_service_properties().await?;
            println!("{info:#?}");
        }
        QueueSubCommands::ListQueues {
            prefix,
            include_metadata,
            max_results,
        } => {
            let mut builder = service_client
                .list_queues()
                .include_metadata(include_metadata);
            args!(builder, prefix, max_results);
            let mut stream = builder.into_stream();
            while let Some(result) = stream.next().await {
                let result = result?;
                for queue in &result.queues {
                    println!("{queue:#?}");
                }
            }
        }
        QueueSubCommands::Create {
            queue_name,
            metadata,
        } => {
            let mut builder = service_client.queue_client(&queue_name).create();
            let metadata = metadata.map(to_metadata);
            args!(builder, metadata);
            let result = builder.await?;
            println!("{result:#?}");
        }
        QueueSubCommands::Delete { queue_name } => {
            let result = service_client.queue_client(&queue_name).delete().await?;
            println!("{result:#?}");
        }
        QueueSubCommands::PutMessage {
            queue_name,
            message,
            visibility_timeout,
            ttl,
        } => {
            let mut builder = service_client
                .queue_client(&queue_name)
                .put_message(&message);
            let visibility_timeout = visibility_timeout.map(Duration::from_secs);
            let ttl = ttl.map(Duration::from_secs);
            args!(builder, visibility_timeout, ttl);
            let result = builder.await?;
            println!("{result:#?}");
        }
        QueueSubCommands::Clear { queue_name } => {
            let result = service_client
                .queue_client(&queue_name)
                .clear_messages()
                .await?;
            println!("{result:#?}");
        }
        QueueSubCommands::GetMessages {
            queue_name,
            number_of_messages,
            visibility_timeout,
        } => {
            let mut builder = service_client.queue_client(&queue_name).get_messages();
            let visibility_timeout = visibility_timeout.map(Duration::from_secs);
            args!(builder, number_of_messages, visibility_timeout);
            let result = builder.await?;
            println!("{result:#?}");
        }
        QueueSubCommands::PopMessage {
            queue_name,
            message_id,
            pop_receipt,
        } => {
            let pop_receipt = PopReceipt::new(message_id, pop_receipt);
            let result = service_client
                .queue_client(&queue_name)
                .pop_receipt_client(pop_receipt)
                .delete()
                .await?;
            println!("{result:#?}");
        }
        QueueSubCommands::PeekMessages {
            queue_name,
            number_of_messages,
        } => {
            let mut builder = service_client.queue_client(&queue_name).peek_messages();
            args!(builder, number_of_messages);
            let result = builder.await?;
            println!("{result:#?}");
        }
    }
    Ok(())
}
