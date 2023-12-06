# everything db
Deployed at `righteous-appliance.testnet`
    
## Functions

### `get_type(type_id: TypeId) -> Option<Type>`

### `new_type(name: String, version: String, value: JsonSchema) -> TypeId`

### `get_thing(thing_id: ThingId) -> Option<Thing>`

### `create_thing(name: String, type_id: TypeId, value: Json) -> ThingId`

### `create_thing(name: String, type_id: TypeId, value: JsonString) -> ThingId`

### `update_thing(thing_id: ThingId, value: String)`

### `delete_thing(thing_id: ThingId)`

## JsonSchema
JsonSchema is a powerful tool for validating the structure and data format of JSON objects, we can use it to validate the types, check [JsonSchema official documentation](https://json-schema.org/learn/getting-started-step-by-step).

# EverythingDB
