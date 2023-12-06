mod things;
mod types;

use crate::things::*;
use crate::types::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::Serialize;
use near_sdk::{
    assert_one_yocto, env, near_bindgen, AccountId, BlockHeight, BorshStorageKey, PanicOnDefault,
};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Types,
    Things,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    types: LookupMap<TypeId, Type>,
    things: LookupMap<ThingId, Thing>,
    types_count: TypeId,
    things_count: ThingId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            types: LookupMap::new(StorageKey::Types),
            things: LookupMap::new(StorageKey::Things),
            types_count: 0,
            things_count: 0,
        }
    }
    pub fn get_type(&self, type_id: TypeId) -> Option<Type> {
        self.types.get(&type_id)
    }
    pub fn get_thing(&self, thing_id: ThingId) -> Option<Thing> {
        self.things.get(&thing_id)
    }
    #[payable]
    pub fn new_type(&mut self, name: String, version: String, value: String) -> TypeId {
        assert_one_yocto();
        let type_id = self.types_count;
        let block_height = env::block_height();
        let creator = env::predecessor_account_id();
        let type_ = Type {
            type_id,
            block_height,
            creator,
            name,
            version,
            value,
        };
        // make sure the type is a valid schema or panic
        assert!(type_.is_valid(), "INVALID_TYPE");
        self.types_count += 1;
        self.types.insert(&type_id, &type_);
        type_id
    }
    #[payable]
    pub fn create_thing(&mut self, name: String, type_id: TypeId, value: String) -> ThingId {
        assert_one_yocto();
        let thing_id = self.things_count;
        let block_height = env::block_height();
        let creator = env::predecessor_account_id();
        let thing = Thing {
            thing_id,
            block_height,
            creator,
            name,
            r#type: type_id,
            value,
        };
        // we should validate that the value is of the correct type
        let type_ = self.types.get(&type_id).unwrap();
        assert!(thing.is_valid(&type_.value), "INVALID_THING");

        self.things_count += 1;
        self.things.insert(&thing_id, &thing);
        thing_id
    }
    #[payable]
    pub fn update_thing(&mut self, thing_id: ThingId, value: String) {
        assert_one_yocto();
        let mut thing = self.things.get(&thing_id).unwrap();
        thing.value = value;
        let type_ = self.types.get(&thing.r#type).unwrap();
        assert!(thing.is_valid(&type_.value), "INVALID_THING");
        self.things.insert(&thing_id, &thing);
    }
    pub fn delete_thing(&mut self, thing_id: ThingId) {
        assert_one_yocto();
        self.things.remove(&thing_id);
    }
}
