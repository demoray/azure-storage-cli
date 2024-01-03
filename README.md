# Azure Storage CLI

A CLI to interact with Azure Storage services

```
Usage: azs [OPTIONS] --account <ACCOUNT> <COMMAND>

Commands:
  account    Interact with the storage account
  container  Interact with storage containers
  queues     
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
  blob    Interact with a blob within a storage container

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
### azs container blob

```
Interact with a blob within a storage container

Usage: blob <BLOB_NAME> <COMMAND>

Commands:
  get                Get the contents of a blob
  get-properties     
  delete             
  put-append-blob    Create a new "append blob" with the contents of the specified file
  append-block       Append the contents of the specified file to an existing "append blob" blob
  create-block-blob  Create a "block blob" with the contents of the specified file
  create-page-blob   Create a "page blob" with the contents of the specified file

Arguments:
  <BLOB_NAME>
          blob name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs container blob get

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
#### azs container blob get-properties

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
#### azs container blob delete

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
#### azs container blob put-append-blob

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
#### azs container blob append-block

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
#### azs container blob create-block-blob

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
#### azs container blob create-page-blob

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
## azs queues

```
Usage: queues <COMMAND>

Commands:
  get-properties  
  list-queues     
  queue           

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queues get-properties

```
Usage: get-properties

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs queues list-queues

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
### azs queues queue

```
Usage: queue <QUEUE_NAME> <COMMAND>

Commands:
  create         
  delete         Delete a Storage Queue
  put-message    Put a message onto the Storage Queue
  clear          Clear all messages on a storage queue
  get-messages   Get messages from a storage queue
  peek-messages  Peek at available messages from a storage queue
  pop-message    Pop a message from a storage queue

Arguments:
  <QUEUE_NAME>
          queue name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue create

```
Usage: create [OPTIONS]

Options:
      --metadata <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue delete

```
Delete a Storage Queue

Usage: delete

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue put-message

```
Put a message onto the Storage Queue

Usage: put-message [OPTIONS] <MESSAGE>

Arguments:
  <MESSAGE>
Options:
      --ttl <TTL>
      --visibility-timeout <VISIBILITY_TIMEOUT>
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue clear

```
Clear all messages on a storage queue

Usage: clear

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue get-messages

```
Get messages from a storage queue

Usage: get-messages [OPTIONS]

Options:
      --number-of-messages <NUMBER_OF_MESSAGES>
      --visibility-timeout <VISIBILITY_TIMEOUT>
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue peek-messages

```
Peek at available messages from a storage queue

Usage: peek-messages [OPTIONS]

Options:
      --number-of-messages <NUMBER_OF_MESSAGES>
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs queues queue pop-message

```
Pop a message from a storage queue

Usage: pop-message <MESSAGE_ID> <POP_RECEIPT>

Arguments:
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
