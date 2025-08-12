## Azure Storage CLI

A CLI to interact with Azure Storage services <ACCOUNT>

```
Usage: azs <ACCOUNT> <COMMAND>

Commands:
  account    Interact with the storage account
  container  Interact with storage containers (and blobs)
  queues     Interact with storage queues
  datalake   Interact with storage datalakes
  tables     Interact with data tables

Arguments:
  <ACCOUNT>
          Storage account name

          [env: STORAGE_ACCOUNT]

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
### azs <ACCOUNT> account

```
Interact with the storage account

Usage: account <COMMAND>

Commands:
  info             Get information about the storage account
  list-containers  List the storage containers in the account

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs <ACCOUNT> account info

```
Get information about the storage account

Usage: info

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs <ACCOUNT> account list-containers

```
List the storage containers in the account

The output of this command is serialized to JSON unless the `show_details` flag is set

Usage: list-containers [OPTIONS]

Options:
      --prefix <PREFIX>
      --include-metadata
      --include-deleted
      --max-results <MAX_RESULTS>
      --show-details
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
#### azs <ACCOUNT> container <CONTAINER_NAME>

```
Interact with storage containers (and blobs)

Usage: container <CONTAINER_NAME> <COMMAND>

Commands:
  create         Create a storage container
  properties     Get properties for a storage container
  delete         Delete a storage container
  list           List blobs in a storage container
  blob           Interact with a blob within a storage container
  generate-sas   Generate a SAS URL for a storage container using the User Deligation Key
  acquire-lease  Acquire a lease on a storage container
  break-lease    Acquire a lease on a storage container
  lease-release
  lease-renew

Arguments:
  <CONTAINER_NAME>
          container name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs <ACCOUNT> container <CONTAINER_NAME> create

```
Create a storage container

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
##### azs <ACCOUNT> container <CONTAINER_NAME> properties

```
Get properties for a storage container

Usage: properties [OPTIONS]

Options:
      --lease-id <LEASE_ID>
          lease id

  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs <ACCOUNT> container <CONTAINER_NAME> delete

```
Delete a storage container

Usage: delete [OPTIONS]

Options:
      --lease-id <LEASE_ID>
          lease id

  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs <ACCOUNT> container <CONTAINER_NAME> list

```
List blobs in a storage container

The output of this command is serialized to JSON unless the `show_details` flag is set

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
      --include-uncommitted-blobs
      --include-copy
      --include-deleted
      --include-tags
      --include-versions
      --show-details
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME>

```
Interact with a blob within a storage container

Usage: blob <BLOB_NAME> <COMMAND>

Commands:
  get                Get the contents of a blob
  get-properties     Get properties of a blob
  delete             Delete a blob
  delete-version-id  Delete the blob at a specific version
  delete-snapsot     Delete the blob at a specific version
  put-append-blob    Create a new "append blob" with the contents of the specified file
  append-block       Append the contents of the specified file to an existing "append blob" blob
  create-block-blob  Create a "block blob" with the contents of the specified file
  create-page-blob   Create a "page blob" with the contents of the specified file
  generate-sas       Generate a SAS URL for the Blob using a User Deligation Key
  get-tags           Get the tags on the blob
  set-tags           Set the tags on the blob
  snapshot           Create a snapshot of the blob
  set-blob-tier      Set the access tier on the blob

Arguments:
  <BLOB_NAME>
          blob name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> get <DESTINATION>

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
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> get-properties

```
Get properties of a blob

Usage: get-properties [OPTIONS]

Options:
      --lease-id <LEASE_ID>
      --if-tags <IF_TAGS>
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> delete

```
Delete a blob

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
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> delete-version-id <VERSION_ID>

```
Delete the blob at a specific version

Usage: delete-version-id [OPTIONS] <VERSION_ID>

Arguments:
  <VERSION_ID>
Options:
      --lease-id <LEASE_ID>
      --permanent
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> delete-snapsot <SNAPSHOT>

```
Delete the blob at a specific version

Usage: delete-snapsot [OPTIONS] <SNAPSHOT>

Arguments:
  <SNAPSHOT>
Options:
      --lease-id <LEASE_ID>
      --permanent
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> put-append-blob

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
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> append-block <PATH>

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
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> create-block-blob <PATH>

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
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> create-page-blob <PATH>

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
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> generate-sas <EXPIRY>

```
Generate a SAS URL for the Blob using a User Deligation Key

Usage: generate-sas [OPTIONS] <--read|--add|--create|--write|--delete|--delete-version|--list|--tags|--move|--execute|--ownership|--permissions|--permanent-delete> <EXPIRY>

Arguments:
  <EXPIRY>
          Expiration

Options:
      --start <START>
          Start time

      --time-format <TIME_FORMAT>
          Format used for the start and expiry times

          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`

          [default: offset]

      --ip <IP>
      --identifier <IDENTIFIER>
      --protocol <PROTOCOL>
          [possible values: https, http-https]

      --read
      --add
      --create
      --write
      --delete
      --delete-version
      --list
      --tags
      --move
      --execute
      --ownership
      --permissions
      --permanent-delete
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> get-tags <IF_TAGS> <LEASE_ID> <SNAPSHOT> <VERSION_ID>

```
Get the tags on the blob

Usage: get-tags [IF_TAGS] [LEASE_ID] [SNAPSHOT] [VERSION_ID]

Arguments:
  [IF_TAGS]
  [LEASE_ID]
  [SNAPSHOT]
  [VERSION_ID]
Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> set-tags <IF_TAGS> <LEASE_ID>

```
Set the tags on the blob

Usage: set-tags [OPTIONS] [IF_TAGS] [LEASE_ID]

Arguments:
  [IF_TAGS]
  [LEASE_ID]
Options:
      --tags <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> snapshot

```
Create a snapshot of the blob

Usage: snapshot [OPTIONS]

Options:
      --unmodified-since <UNMODIFIED_SINCE>
      --modified-since <MODIFIED_SINCE>
      --time-format <TIME_FORMAT>
          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`[default: TimeFormat::Offset]

      --if-tags <IF_TAGS>
      --lease-id <LEASE_ID>
      --metadata <KEY=VALUE>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> blob <BLOB_NAME> set-blob-tier

```
Set the access tier on the blob

Usage: set-blob-tier [OPTIONS] --tier <TIER>

Options:
      --tier <TIER>
      --rehydrate-priority <REHYDRATE_PRIORITY>
      --if-tags <IF_TAGS>
      --snapshot <SNAPSHOT>
      --version-id <VERSION_ID>
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> generate-sas <EXPIRY>

```
Generate a SAS URL for a storage container using the User Deligation Key

Usage: generate-sas [OPTIONS] <--read|--add|--create|--write|--delete|--delete-version|--list|--tags|--move|--execute|--ownership|--permissions|--permanent-delete> <EXPIRY>

Arguments:
  <EXPIRY>
          Expiration

Options:
      --start <START>
          Start time

      --time-format <TIME_FORMAT>
          Format used for the start and expiry times

          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`

          [default: offset]

      --ip <IP>
      --identifier <IDENTIFIER>
      --protocol <PROTOCOL>
          [possible values: https, http-https]

      --read
      --add
      --create
      --write
      --delete
      --delete-version
      --list
      --tags
      --move
      --execute
      --ownership
      --permissions
      --permanent-delete
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> acquire-lease <LEASE_DURATION> <PROPOSED_LEASE_ID> <LEASE_ID> <UNMODIFIED_SINCE> <MODIFIED_SINCE>

```
Acquire a lease on a storage container

Usage: acquire-lease [OPTIONS] [LEASE_DURATION] [PROPOSED_LEASE_ID] [LEASE_ID] [UNMODIFIED_SINCE] [MODIFIED_SINCE]

Arguments:
  [LEASE_DURATION]
          lease duration in seconds (otherwise uses Infinite)

  [PROPOSED_LEASE_ID]
  [LEASE_ID]
  [UNMODIFIED_SINCE]
  [MODIFIED_SINCE]
Options:
      --time-format <TIME_FORMAT>
          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`[default: TimeFormat::Offset]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> break-lease <LEASE_BREAK_PERIOD> <LEASE_ID> <UNMODIFIED_SINCE> <MODIFIED_SINCE>

```
Acquire a lease on a storage container

Usage: break-lease [OPTIONS] [LEASE_BREAK_PERIOD] [LEASE_ID] [UNMODIFIED_SINCE] [MODIFIED_SINCE]

Arguments:
  [LEASE_BREAK_PERIOD]
          Duration as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`

  [LEASE_ID]
  [UNMODIFIED_SINCE]
  [MODIFIED_SINCE]
Options:
      --time-format <TIME_FORMAT>
          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`[default: TimeFormat::Offset]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> lease-release <LEASE_ID> <UNMODIFIED_SINCE> <MODIFIED_SINCE>

```
Usage: lease-release [OPTIONS] <LEASE_ID> [UNMODIFIED_SINCE] [MODIFIED_SINCE]

Arguments:
  <LEASE_ID>
  [UNMODIFIED_SINCE]
  [MODIFIED_SINCE]
Options:
      --time-format <TIME_FORMAT>
          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`[default: TimeFormat::Offset]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> container <CONTAINER_NAME> lease-renew <LEASE_ID> <UNMODIFIED_SINCE> <MODIFIED_SINCE>

```
Usage: lease-renew [OPTIONS] <LEASE_ID> [UNMODIFIED_SINCE] [MODIFIED_SINCE]

Arguments:
  <LEASE_ID>
  [UNMODIFIED_SINCE]
  [MODIFIED_SINCE]
Options:
      --time-format <TIME_FORMAT>
          Possible values:
          - rfc3339: Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>. Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
          - offset:  Offset from `now`, as parsed by <https://docs.rs/duration-string/latest/duration_string/> Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`[default: TimeFormat::Offset]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
### azs <ACCOUNT> queues

```
Interact with storage queues

Usage: queues <COMMAND>

Commands:
  get-properties
  list-queues     List the available queues
  queue

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs <ACCOUNT> queues get-properties

```
Usage: get-properties

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs <ACCOUNT> queues list-queues

```
List the available queues

The output of this command is serialized as JSON

Usage: list-queues [OPTIONS]

Options:
      --prefix <PREFIX>
      --include-metadata
      --max-results <MAX_RESULTS>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
##### azs <ACCOUNT> queues queue <QUEUE_NAME>

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
###### azs <ACCOUNT> queues queue <QUEUE_NAME> create

```
Usage: create [OPTIONS]

Options:
      --metadata <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> queues queue <QUEUE_NAME> delete

```
Delete a Storage Queue

Usage: delete

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> queues queue <QUEUE_NAME> put-message <MESSAGE>

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
###### azs <ACCOUNT> queues queue <QUEUE_NAME> clear

```
Clear all messages on a storage queue

Usage: clear

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> queues queue <QUEUE_NAME> get-messages

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
###### azs <ACCOUNT> queues queue <QUEUE_NAME> peek-messages

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
###### azs <ACCOUNT> queues queue <QUEUE_NAME> pop-message <MESSAGE_ID> <POP_RECEIPT>

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
### azs <ACCOUNT> datalake

```
Interact with storage datalakes

Usage: datalake <COMMAND>

Commands:
  list-file-systems  List available file systems
  file-system

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs <ACCOUNT> datalake list-file-systems

```
List available file systems

The output of this command is serialized as JSON

Usage: list-file-systems [OPTIONS]

Options:
      --prefix <PREFIX>
      --max-results <MAX_RESULTS>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
##### azs <ACCOUNT> datalake file-system <NAME>

```
Usage: file-system <NAME> <COMMAND>

Commands:
  create      Create the specified filesystem
  delete      Create the specified filesystem
  list-paths  List paths in the specified file system
  directory   Perform operations on the specified directory

Arguments:
  <NAME>
Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> create

```
Create the specified filesystem

Usage: create [OPTIONS]

Options:
      --properties <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> delete

```
Create the specified filesystem

Usage: delete

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> list-paths

```
List paths in the specified file system

The output of this command is serialized as JSON

Usage: list-paths [OPTIONS]

Options:
      --recursive <RECURSIVE>
          [possible values: true, false]

      --max-results <MAX_RESULTS>
      --upn <UPN>
          [possible values: true, false]

      --directory <DIRECTORY>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> directory <DIRECTORY_NAME>

```
Perform operations on the specified directory

Usage: directory <DIRECTORY_NAME> <COMMAND>

Commands:
  create
  delete
  list-paths  List available paths

Arguments:
  <DIRECTORY_NAME>
Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> directory <DIRECTORY_NAME> create

```
Usage: create [OPTIONS]

Options:
      --properties <KEY=VALUE>
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> directory <DIRECTORY_NAME> delete

```
Usage: delete [OPTIONS]

Options:
      --recursive
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> datalake file-system <NAME> directory <DIRECTORY_NAME> list-paths

```
List available paths

The output of this command is serialized as JSON

Usage: list-paths [OPTIONS]

Options:
      --recursive <RECURSIVE>
          [possible values: true, false]

      --max-results <MAX_RESULTS>
      --upn <UPN>
          [possible values: true, false]

      --directory <DIRECTORY>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
### azs <ACCOUNT> tables

```
Interact with data tables

Usage: tables <COMMAND>

Commands:
  list               List available tables
  create             Create a new table
  delete             Delete a table
  query              Query a table
  get                Get a specific row in the table
  insert-or-merge
  insert-or-replace
  delete-entity
  update-entity
  merge-entity

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
#### azs <ACCOUNT> tables list

```
List available tables

The output of this command is serialized as JSON

Usage: list [OPTIONS]

Options:
      --filter <FILTER>
      --select <SELECT>
      --top <TOP>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
##### azs <ACCOUNT> tables create <TABLE_NAME>

```
Create a new table

Usage: create <TABLE_NAME>

Arguments:
  <TABLE_NAME>
          table name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs <ACCOUNT> tables delete <TABLE_NAME>

```
Delete a table

Usage: delete <TABLE_NAME>

Arguments:
  <TABLE_NAME>
          table name

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
##### azs <ACCOUNT> tables query <TABLE_NAME>

```
Query a table

The output of this command is serialized as JSON

Usage: query [OPTIONS] <TABLE_NAME>

Arguments:
  <TABLE_NAME>
          table name

Options:
      --filter <FILTER>
      --select <SELECT>
      --top <TOP>
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> tables get <TABLE_NAME> <PARTITION_KEY> <ROW_KEY>

```
Get a specific row in the table

The output of this command is serialized as JSON

Usage: get <TABLE_NAME> <PARTITION_KEY> <ROW_KEY>

Arguments:
  <TABLE_NAME>
          table name

  <PARTITION_KEY>
          Partition Key

  <ROW_KEY>
          Row Key

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
###### azs <ACCOUNT> tables insert-or-merge <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

```
Usage: insert-or-merge <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

Arguments:
  <TABLE_NAME>
          table name

  <PARTITION_KEY>
          Partition Key

  <ROW_KEY>
          Row Key

  <JSON_FILE>
          JSON file containing the entity

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> tables insert-or-replace <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

```
Usage: insert-or-replace <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

Arguments:
  <TABLE_NAME>
          table name

  <PARTITION_KEY>
          Partition Key

  <ROW_KEY>
          Row Key

  <JSON_FILE>
          JSON file containing the entity

Options:
  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> tables delete-entity <TABLE_NAME> <PARTITION_KEY> <ROW_KEY>

```
Usage: delete-entity [OPTIONS] <TABLE_NAME> <PARTITION_KEY> <ROW_KEY>

Arguments:
  <TABLE_NAME>
          table name

  <PARTITION_KEY>
          Partition Key

  <ROW_KEY>
          Row Key

Options:
      --if-match-condition <IF_MATCH_CONDITION>
          `ETag` value

  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> tables update-entity <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

```
Usage: update-entity [OPTIONS] <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

Arguments:
  <TABLE_NAME>
          table name

  <PARTITION_KEY>
          Partition Key

  <ROW_KEY>
          Row Key

  <JSON_FILE>
          JSON file containing the entity

Options:
      --if-match-condition <IF_MATCH_CONDITION>
          `ETag` value

  -h, --help
          Print help

  -V, --version
          Print version

```
###### azs <ACCOUNT> tables merge-entity <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

```
Usage: merge-entity [OPTIONS] <TABLE_NAME> <PARTITION_KEY> <ROW_KEY> <JSON_FILE>

Arguments:
  <TABLE_NAME>
          table name

  <PARTITION_KEY>
          Partition Key

  <ROW_KEY>
          Row Key

  <JSON_FILE>
          JSON file containing the entity

Options:
      --if-match-condition <IF_MATCH_CONDITION>
          `ETag` value

  -h, --help
          Print help

  -V, --version
          Print version

```