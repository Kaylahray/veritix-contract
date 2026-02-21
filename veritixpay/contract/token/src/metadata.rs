use soroban_sdk::{contracttype, Env, String};

use crate::storage_types::DataKey;

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimal: u32,
}

pub fn read_metadata(e: &Env) -> TokenMetadata {
    e.storage().instance().get(&DataKey::Metadata).unwrap()
}

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    e.storage().instance().set(&DataKey::Metadata, &metadata);
}

pub fn read_decimal(e: &Env) -> u32 {
    read_metadata(e).decimal
}

pub fn read_name(e: &Env) -> String {
    read_metadata(e).name
}

pub fn read_symbol(e: &Env) -> String {
    read_metadata(e).symbol
}
