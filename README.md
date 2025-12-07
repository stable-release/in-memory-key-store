This README is here to remind myself how this persisted REPL key-value storage works.

- Supported command handlers:
    1. set
    2. get
    3. list
    4. delete
    5. clear
    6. exit

- Key:Value pairs are loaded into memory as a hashmap from a local json file
    - serde_json serializes and deserializes into Value (which is nearly type agnostic)

- Files are persisted to local_storage.json by:
    1. Writing the store in memory k:v hashmap to a new "_overwrite.json" file
    2. Deleting the old local_storage.json
    3. Renaming the "_overwrite" to the "local_storage.json"
