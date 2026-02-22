#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

use crate::VeritixTokenClient;

fn setup() -> (Env, VeritixTokenClient, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, VeritixToken);
    let client = VeritixTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    (env, client, admin, user)
}

#[test]
fn test_initialize() {
    let (_env, client, admin, _) = setup();

    client.initialize(
        &admin,
        &String::from_str(&_env, "Veritix"),
        &String::from_str(&_env, "VTX"),
        &7u32,
    );

    assert_eq!(client.name(), String::from_str(&_env, "Veritix"));
    assert_eq!(client.symbol(), String::from_str(&_env, "VTX"));
    assert_eq!(client.decimals(), 7u32);
}

#[test]
#[should_panic]
fn test_initialize_twice_panics() {
    let (env, client, admin, _) = setup();

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    // Second initialize must panic
    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );
}

#[test]
fn test_mint() {
    let (env, client, admin, user) = setup();

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &1000i128);

    assert_eq!(client.balance(&user), 1000i128);
}

#[test]
#[should_panic]
fn test_mint_unauthorized_panics() {
    let (env, client, admin, user) = setup();

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    // Unauthorized user attempts mint
    client.mint(&user, &user, &1000i128);
}

#[test]
fn test_burn() {
    let (env, client, admin, user) = setup();

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &1000i128);
    client.burn(&user, &500i128);

    assert_eq!(client.balance(&user), 500i128);
}

#[test]
#[should_panic]
fn test_burn_insufficient_panics() {
    let (env, client, admin, user) = setup();

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &100i128);

    client.burn(&user, &200i128);
}

#[test]
fn test_transfer() {
    let (env, client, admin, user) = setup();
    let receiver = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &1000i128);

    client.transfer(&user, &receiver, &400i128);

    assert_eq!(client.balance(&user), 600i128);
    assert_eq!(client.balance(&receiver), 400i128);
}

#[test]
#[should_panic]
fn test_transfer_insufficient_balance_panics() {
    let (env, client, admin, user) = setup();
    let receiver = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.transfer(&user, &receiver, &100i128);
}

#[test]
fn test_transfer_from() {
    let (env, client, admin, user) = setup();
    let spender = Address::generate(&env);
    let receiver = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &1000i128);

    client.approve(&user, &spender, &500i128, &1000u32);
    client.transfer_from(&spender, &user, &receiver, &300i128);

    assert_eq!(client.balance(&receiver), 300i128);
}

#[test]
fn test_approve_and_spend_allowance() {
    let (env, client, admin, user) = setup();
    let spender = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &1000i128);

    client.approve(&user, &spender, &400i128, &1000u32);
    client.transfer_from(&spender, &user, &spender, &200i128);

    assert_eq!(client.balance(&spender), 200i128);
}

#[test]
#[should_panic]
fn test_expired_allowance_panics() {
    let (env, client, admin, user) = setup();
    let spender = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Veritix"),
        &String::from_str(&env, "VTX"),
        &7u32,
    );

    client.mint(&admin, &user, &1000i128);

    // Expired immediately (0 ledger)
    client.approve(&user, &spender, &400i128, &0u32);

    client.transfer_from(&spender, &user, &spender, &100i128);
}