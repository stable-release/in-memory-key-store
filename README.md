# A parallel in-memory key-value store with a REPL and local persistence.

## Supported command handlers in cli-REPL:
    - set
    - get
    - list
    - delete
    - clear
    - exit

## Key:Value pairs are loaded into memory as a hashmap from a local json file
    - serde_json serializes and deserializes into Value (which is nearly type agnostic)

## Files are persisted to local_storage.json by:
    - Writing the store in memory k:v hashmap to a new "_overwrite.json" file
    - Deleting the old local_storage.json
    - Renaming the "_overwrite" to the "local_storage.json"
