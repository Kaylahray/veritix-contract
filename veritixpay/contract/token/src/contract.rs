use crate::admin::{check_admin, has_admin, write_admin};
use crate::allowance::{read_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::DataKey;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use crate::freeze::{freeze_account, is_frozen, unfreeze_account};

#[contract]
pub struct VeritixToken;

#[contractimpl]
impl VeritixToken {

    // --- NEW ADMIN FUNCTIONS ---
    
    pub fn freeze(e: Env, target: Address) {
        crate::admin::check_admin(&e);
        let admin = crate::admin::read_admin(&e);
        freeze_account(&e, admin, target);
    }

    pub fn unfreeze(e: Env, target: Address) {
        crate::admin::check_admin(&e);
        let admin = crate::admin::read_admin(&e);
        unfreeze_account(&e, admin, target);
    }

    // --- UPDATED TOKEN FUNCTIONS ---

    pub fn burn(e: Env, from: Address, amount: i128) {
        if is_frozen(&e, &from) {
            panic!("account frozen");
        }
        from.require_auth();
        spend_balance(&e, from.clone(), amount);
        e.events().publish((symbol_short!("burn"), from), amount);
    }

    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        if is_frozen(&e, &from) {
            panic!("account frozen");
        }
        from.require_auth();
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        e.events().publish((symbol_short!("transfer"), from, to), amount);
    }

    pub fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        if is_frozen(&e, &from) {
            panic!("account frozen");
        }
        spender.require_auth();
        let allowance = read_allowance(&e, from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }
        write_allowance(&e, from.clone(), spender, allowance - amount, e.ledger().sequence() + 100);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        e.events().publish((symbol_short!("transfer"), from, to), amount);
    }

    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if has_admin(&e) {
            panic!("already initialized");
        }
        write_admin(&e, &admin);
        write_metadata(&e, decimal, name, symbol);
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_admin(&e); // Admin-only check
        e.storage().instance().extend_ttl(100, 100);
        receive_balance(&e, to, amount);
    }

    pub fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();
        spend_balance(&e, from, amount);
    }

    pub fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        let allowance = read_allowance(&e, from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }
        write_allowance(&e, from.clone(), spender, allowance - amount);
        spend_balance(&e, from, amount);
    }

    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        spend_balance(&e, from, to.clone(), amount);
        receive_balance(&e, to, amount);
    }

    pub fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let allowance = read_allowance(&e, from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }
        write_allowance(&e, from.clone(), spender, allowance - amount);
        spend_balance(&e, from, amount);
        receive_balance(&e, to, amount);
    }

    pub fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        write_allowance(&e, from, spender, amount, expiration_ledger);
    }

    // --- Read-Only Functions ---

    pub fn balance(e: Env, id: Address) -> i128 {
        read_balance(&e, id)
    }

    pub fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        read_allowance(&e, from, spender)
    }

    pub fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    pub fn name(e: Env) -> String {
        read_name(&e)
    }

    pub fn symbol(e: Env) -> String {
        read_symbol(&e)
    }

pub fn set_admin(e: Env, new_admin: Address) {
    crate::admin::transfer_admin(&e, new_admin);
}
}