# azure-blob-storage-cli

```
Usage: azure-blob-storage-cli <ACCOUNT> <ACCESS_KEY> <COMMAND>

Commands:
  account    Interact with the storage account
  container  Interact with storage containers
  blob       Interact with a blob within a storage container

Arguments:
  <ACCOUNT>
          storage account name
          
          [env: STORAGE_ACCOUNT=]

  <ACCESS_KEY>
          storage account access key
          
          [env: STORAGE_ACCESS_KEY=]

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
## azure-blob-storage-cli account

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
### azure-blob-storage-cli account info

```
Usage: info

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azure-blob-storage-cli account list-containers

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
## azure-blob-storage-cli container

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
### azure-blob-storage-cli container create

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
### azure-blob-storage-cli container delete

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
### azure-blob-storage-cli container list

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
## azure-blob-storage-cli blob

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
### azure-blob-storage-cli blob get

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
### azure-blob-storage-cli blob get-properties

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
### azure-blob-storage-cli blob delete

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
### azure-blob-storage-cli blob put-append-blob

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
### azure-blob-storage-cli blob append-block

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
### azure-blob-storage-cli blob create-block-blob

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
### azure-blob-storage-cli blob create-page-blob

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
