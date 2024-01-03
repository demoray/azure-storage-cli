use crate::args;
use crate::utils::{parse_key_val, to_metadata};
use azure_storage_queues::{PopReceipt, QueueServiceClient, QueueClient};
use clap::Subcommand;
use futures::StreamExt;
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Subcommand)]
pub enum QueuesSubCommands {
    GetProperties,
    ListQueues {
        #[clap(long)]
        prefix: Option<String>,
        #[clap(long)]
        include_metadata: bool,
        #[clap(long)]
        max_results: Option<NonZeroU32>,
    },
    Queue {
        /// queue name
        queue_name: String,
        #[clap(subcommand)]
        subcommand: IndividualQueueSubCommands,
    }
}

pub async fn queues_commands(
    service_client: &QueueServiceClient,
    subcommand: QueuesSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        QueuesSubCommands::GetProperties => {
            let info = service_client.get_queue_service_properties().await?;
            println!("{info:#?}");
        }
        QueuesSubCommands::ListQueues {
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
        QueuesSubCommands::Queue {
            queue_name,
            subcommand,
        } => {
            let queue_client = service_client.queue_client(&queue_name);
            individual_queue_commands(&queue_client, subcommand).await?;
        }
    }
    Ok(())
}

#[derive(Subcommand)]
pub enum IndividualQueueSubCommands {
     Create {
        #[clap(long, value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = clap::ArgAction::Append)]
        metadata: Option<Vec<(String, String)>>,
    },
    /// Delete a Storage Queue
    Delete,
    /// Put a message onto the Storage Queue
    PutMessage {
        message: String,
        // TTL (in seconds)
        #[clap(long)]
        ttl: Option<u64>,
        // Visibility timeout (in seconds)
        #[clap(long)]
        visibility_timeout: Option<u64>,
    },
    /// Clear all messages on a storage queue
    Clear,
    /// Get messages from a storage queue
    GetMessages {
        #[clap(long)]
        number_of_messages: Option<u8>,
        #[clap(long)]
        // Visibility timeout (in seconds)
        visibility_timeout: Option<u64>,
    },
    /// Peek at available messages from a storage queue
    PeekMessages {
        #[clap(long)]
        number_of_messages: Option<u8>,
    },
    /// Pop a message from a storage queue
    PopMessage {
        /// Message ID for the message to be deleted (usually from the `GetMessages` response)
        message_id: String,
        /// Pop Receipt the message to be deleted (usually from the `GetMessages` response)
        pop_receipt: String,
    },
}

pub async fn individual_queue_commands(
    queue_client: &QueueClient,
    subcommand: IndividualQueueSubCommands,
) -> azure_core::Result<()> {
    match subcommand {
        IndividualQueueSubCommands::Create {
            metadata,
        } => {
            let mut builder = queue_client.create();
            let metadata = metadata.map(to_metadata);
            args!(builder, metadata);
            let result = builder.await?;
            println!("{result:#?}");
        }
        IndividualQueueSubCommands::Delete => {
            let result = queue_client.delete().await?;
            println!("{result:#?}");
        }
        IndividualQueueSubCommands::PutMessage {
            message,
            visibility_timeout,
            ttl,
        } => {
            let mut builder = queue_client.put_message(&message);
            let visibility_timeout = visibility_timeout.map(Duration::from_secs);
            let ttl = ttl.map(Duration::from_secs);
            args!(builder, visibility_timeout, ttl);
            let result = builder.await?;
            println!("{result:#?}");
        }
        IndividualQueueSubCommands::Clear => {
            let result = queue_client .clear_messages()
                .await?;
            println!("{result:#?}");
        }
        IndividualQueueSubCommands::GetMessages {
            number_of_messages,
            visibility_timeout,
        } => {
            let mut builder = queue_client.get_messages();
            let visibility_timeout = visibility_timeout.map(Duration::from_secs);
            args!(builder, number_of_messages, visibility_timeout);
            let result = builder.await?;
            println!("{result:#?}");
        }
        IndividualQueueSubCommands::PopMessage {
            message_id,
            pop_receipt,
        } => {
            let pop_receipt = PopReceipt::new(message_id, pop_receipt);
            let result = queue_client.pop_receipt_client(pop_receipt)
                .delete()
                .await?;
            println!("{result:#?}");
        }
        IndividualQueueSubCommands::PeekMessages {
            number_of_messages,
        } => {
            let mut builder = queue_client.peek_messages();
            args!(builder, number_of_messages);
            let result = builder.await?;
            println!("{result:#?}");
        }
    }
    Ok(())
}
