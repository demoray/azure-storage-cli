# Azure Storage CLI

A CLI to interact with Azure Storage services

```
Usage: azs [OPTIONS] --account <ACCOUNT> <COMMAND>

Commands:
  account    Interact with the storage account
  container  Interact with storage containers
  blob       Interact with a blob within a storage container
  queue      
  datalake   

Options:
      --account <ACCOUNT>
          storage account name.  Set the environment variable STORAGE_ACCOUNT to set a default
          
          [env: STORAGE_ACCOUNT]

      --use-default-credentials
      --access-key <ACCESS_KEY>
          storage account access key.  If not set, authentication will be done via Azure Entra Id using the `DefaultAzureCredential` (see https://docs.rs/azure_identity/latest/azure_identity/struct.DefaultAzureCredential.html)
          
          [env: STORAGE_ACCESS_KEY]

  -h, --help
          Print help

  -V, --version
          Print version

```
## azs account

```
Interact with the storage account

Usage: account <COMMAND>

Commands:
  info             
  list-containers  

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs account info

```
Usage: info

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs account list-containers

```
Usage: list-containers [OPTIONS]

Options:
      --prefix <PREFIX>
      --include-metadata
      --include-deleted
      --max-results <MAX_RESULTS>
  -h, --help
          Print help

  -V, --version
          Print version

```
## azs container

```
Interact with storage containers

Usage: container <CONTAINER_NAME> <COMMAND>

Commands:
  create  
  delete  
  list    

Arguments:
  <CONTAINER_NAME>
          container name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs container create

```
Usage: create [OPTIONS]

Options:
      --public-access <PUBLIC_ACCESS>
          public access level

      --metadata <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs container delete

```
Usage: delete [OPTIONS]

Options:
      --lease-id <LEASE_ID>
          lease id

  -h, --help
          Print help

  -V, --version
          Print version

```
### azs container list

```
Usage: list [OPTIONS]

Options:
      --prefix <PREFIX>
          only include blobs with the specified prefix

      --delimiter <DELIMITER>
          only include blobs with the specified delimiter

      --max-results <MAX_RESULTS>
          max results to return

      --include-snapshots
      --include-metadata
      --include-uncommited-blobs
      --include-copy
      --include-deleted
      --include-tags
      --include-versions
  -h, --help
          Print help

  -V, --version
          Print version

```
## azs blob

```
Interact with a blob within a storage container

Usage: blob <CONTAINER_NAME> <BLOB_NAME> <COMMAND>

Commands:
  get                Get the contents of a blob
  get-properties     
  delete             
  put-append-blob    Create a new "append blob" with the contents of the specified file
  append-block       Append the contents of the specified file to an existing "append blob" blob
  create-block-blob  Create a "block blob" with the contents of the specified file
  create-page-blob   Create a "page blob" with the contents of the specified file

Arguments:
  <CONTAINER_NAME>
          container name

  <BLOB_NAME>
          blob name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob get

```
Get the contents of a blob

Usage: get [OPTIONS] [DESTINATION]

Arguments:
  [DESTINATION]
          Where should the contents of the file be written (otherwise, written to STDOUT)

Options:
      --lease-id <LEASE_ID>
      --chunk-size <CHUNK_SIZE>
      --if-tags <IF_TAGS>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob get-properties

```
Usage: get-properties [OPTIONS]

Options:
      --lease-id <LEASE_ID>
      --if-tags <IF_TAGS>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob delete

```
Usage: delete [OPTIONS]

Options:
      --lease-id <LEASE_ID>
      --if-tags <IF_TAGS>
      --delete-snapshots-method <DELETE_SNAPSHOTS_METHOD>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob put-append-blob

```
Create a new "append blob" with the contents of the specified file

Usage: put-append-blob [OPTIONS]

Options:
      --content-type <CONTENT_TYPE>
      --content-encoding <CONTENT_ENCODING>
      --content-language <CONTENT_LANGUAGE>
      --content-disposition <CONTENT_DISPOSITION>
      --tags <KEY=VALUE>
      --metadata <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob append-block

```
Append the contents of the specified file to an existing "append blob" blob

Usage: append-block [OPTIONS] <PATH>

Arguments:
  <PATH>
Options:
      --condition-max-size <CONDITION_MAX_SIZE>
      --condition-append-position <CONDITION_APPEND_POSITION>
      --if-tags <IF_TAGS>
      --lease-id <LEASE_ID>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob create-block-blob

```
Create a "block blob" with the contents of the specified file

Usage: create-block-blob [OPTIONS] <PATH>

Arguments:
  <PATH>
Options:
      --upload-block-size <UPLOAD_BLOCK_SIZE>
          Upload the file in blocks of this size

      --buffer-size <BUFFER_SIZE>
          How much to buffer in memory while uploading

      --content-type <CONTENT_TYPE>
      --content-encoding <CONTENT_ENCODING>
      --content-language <CONTENT_LANGUAGE>
      --content-disposition <CONTENT_DISPOSITION>
      --tags <KEY=VALUE>
      --metadata <KEY=VALUE>
      --if-tags <IF_TAGS>
      --lease-id <LEASE_ID>
      --access-tier <ACCESS_TIER>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs blob create-page-blob

```
Create a "page blob" with the contents of the specified file

Usage: create-page-blob [OPTIONS] <PATH>

Arguments:
  <PATH>
Options:
      --content-type <CONTENT_TYPE>
      --content-encoding <CONTENT_ENCODING>
      --content-language <CONTENT_LANGUAGE>
      --content-disposition <CONTENT_DISPOSITION>
      --tags <KEY=VALUE>
      --metadata <KEY=VALUE>
      --lease-id <LEASE_ID>
      --sequence-number <SEQUENCE_NUMBER>
      --upload-block-size <UPLOAD_BLOCK_SIZE>
  -h, --help
          Print help

  -V, --version
          Print version

```
## azs queue

```
Usage: queue <COMMAND>

Commands:
  get-properties  
  list-queues     
  create          Create a Storage Queue
  delete          Delete a Storage Queue
  put-message     Put a message onto the Storage Queue
  clear           Clear all messages on a storage queue
  get-messages    Get messages from a storage queue
  peek-messages   Peek at available messages from a storage queue
  pop-message     Pop a message from a storage queue

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue get-properties

```
Usage: get-properties

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue list-queues

```
Usage: list-queues [OPTIONS]

Options:
      --prefix <PREFIX>
      --include-metadata
      --max-results <MAX_RESULTS>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue create

```
Create a Storage Queue

Usage: create [OPTIONS] <QUEUE_NAME>

Arguments:
  <QUEUE_NAME>
          Name of the queue

Options:
      --metadata <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue delete

```
Delete a Storage Queue

Usage: delete <QUEUE_NAME>

Arguments:
  <QUEUE_NAME>
          Name of the queue

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue put-message

```
Put a message onto the Storage Queue

Usage: put-message [OPTIONS] <QUEUE_NAME> <MESSAGE>

Arguments:
  <QUEUE_NAME>
          Name of the queue

  <MESSAGE>
Options:
      --ttl <TTL>
      --visibility-timeout <VISIBILITY_TIMEOUT>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue clear

```
Clear all messages on a storage queue

Usage: clear <QUEUE_NAME>

Arguments:
  <QUEUE_NAME>
          Name of the queue

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue get-messages

```
Get messages from a storage queue

Usage: get-messages [OPTIONS] <QUEUE_NAME>

Arguments:
  <QUEUE_NAME>
          Name of the queue

Options:
      --number-of-messages <NUMBER_OF_MESSAGES>
      --visibility-timeout <VISIBILITY_TIMEOUT>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue peek-messages

```
Peek at available messages from a storage queue

Usage: peek-messages [OPTIONS] <QUEUE_NAME>

Arguments:
  <QUEUE_NAME>
          Name of the queue

Options:
      --number-of-messages <NUMBER_OF_MESSAGES>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queue pop-message

```
Pop a message from a storage queue

Usage: pop-message <QUEUE_NAME> <MESSAGE_ID> <POP_RECEIPT>

Arguments:
  <QUEUE_NAME>
          Name of the queue

  <MESSAGE_ID>
          Message ID for the message to be deleted (usually from the `GetMessages` response)

  <POP_RECEIPT>
          Pop Receipt the message to be deleted (usually from the `GetMessages` response)

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
## azs datalake

```
Usage: datalake <COMMAND>

Commands:
  list-file-systems  
  file-system        

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs datalake list-file-systems

```
Usage: list-file-systems [OPTIONS]

Options:
      --prefix <PREFIX>
      --max-results <MAX_RESULTS>
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs datalake file-system

```
Usage: file-system <NAME> <COMMAND>

Commands:
  create      
  delete      
  list-paths  
  directory   

Arguments:
  <NAME>
Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs datalake file-system create

```
Usage: create [OPTIONS]

Options:
      --properties <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs datalake file-system delete

```
Usage: delete

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs datalake file-system list-paths

```
Usage: list-paths [OPTIONS]

Options:
      --recursive <RECURSIVE>
          [possible values: true, false]

      --max-results <MAX_RESULTS>
      --upn <UPN>
          [possible values: true, false]

      --directory <DIRECTORY>
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs datalake file-system directory

```
Usage: directory <DIRECTORY_NAME> <COMMAND>

Commands:
  create      
  delete      
  list-paths  

Arguments:
  <DIRECTORY_NAME>
Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs datalake file-system directory create

```
Usage: create [OPTIONS]

Options:
      --properties <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs datalake file-system directory delete

```
Usage: delete [OPTIONS]

Options:
      --recursive
  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs datalake file-system directory list-paths

```
Usage: list-paths [OPTIONS]

Options:
      --recursive <RECURSIVE>
          [possible values: true, false]

      --max-results <MAX_RESULTS>
      --upn <UPN>
          [possible values: true, false]

      --directory <DIRECTORY>
  -h, --help
          Print help

  -V, --version
          Print version

```
