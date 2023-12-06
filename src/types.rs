use crate::*;

use jsonschema_valid::{schemas, Config};
use near_sdk::serde_json::{self, Value};
use schemars::JsonSchema;

pub type TypeId = u128;

#[derive(JsonSchema, BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Type {
    pub type_id: TypeId,
    pub block_height: BlockHeight,
    pub creator: AccountId,
    pub name: String,
    pub version: String,
    pub value: String,
}

impl Type {
    pub fn is_valid(&self) -> bool {
        let schema_json: Value = serde_json::from_str(&self.value).unwrap();
        Config::from_schema(&schema_json, Some(schemas::Draft::Draft6)).is_ok()
    }
}
