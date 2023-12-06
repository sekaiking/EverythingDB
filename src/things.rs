use crate::*;

use jsonschema_valid::{schemas, Config};
use near_sdk::serde_json::{self, Value};
use schemars::JsonSchema;

pub type ThingId = u128;

#[derive(JsonSchema, BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Thing {
    pub thing_id: ThingId,
    pub block_height: u64,
    pub creator: AccountId,
    pub name: String,
    pub r#type: u128,
    pub value: String,
}

impl Thing {
    pub fn is_valid(&self, schema: &str) -> bool {
        let schema_json: Value = serde_json::from_str(schema).unwrap();
        let instance: Value = serde_json::from_str(&self.value).unwrap();
        let ok = Config::from_schema(&schema_json, Some(schemas::Draft::Draft6))
            .unwrap()
            .validate(&instance)
            .is_ok();

        ok
    }
}
