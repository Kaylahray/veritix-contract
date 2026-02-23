use crate::balance::{receive_balance, spend_balance};
use crate::storage_types::DataKey;
use soroban_sdk::{contracttype, Address, Env, Symbol};

use crate::splitter::SplitRecipient;
use crate::admin::read_admin; // Assuming read_admin returns the Admin Address
use soroban_sdk::Vec;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowRecord {
    pub id: u32,
    pub depositor: Address,
    pub beneficiary: Address,
    pub amount: i128,
    pub released: bool,
    pub refunded: bool,
    pub expiration_ledger: u32,
    pub release_after_ledger: u32,
}

/// Creates a new escrow record and locks the funds in the contract.
pub fn create_escrow(
    e: &Env,
    depositor: Address,
    beneficiary: Address,
    amount: i128,
    expiration_ledger: u32,
    release_after_ledger: u32,
) -> u32 {
    depositor.require_auth();

    // 1. Move funds from the depositor to the contract itself
    spend_balance(e, depositor.clone(), amount);
    receive_balance(e, e.current_contract_address(), amount);

    // 2. Increment and fetch the new Escrow ID
    let mut count: u32 = e.storage().instance().get(&DataKey::EscrowCount).unwrap_or(0);
    count += 1;
    e.storage().instance().set(&DataKey::EscrowCount, &count);

    // 3. Store the record
    let record = EscrowRecord {
        id: count,
        depositor: depositor.clone(),
        beneficiary: beneficiary.clone(),
        amount,
        released: false,
        refunded: false,
        expiration_ledger,
        release_after_ledger,
    };
    e.storage().persistent().set(&DataKey::Escrow(count), &record);

    // 4. Emit Event
    e.events().publish(
        (Symbol::new(e, "escrow"), Symbol::new(e, "created"), depositor),
        (beneficiary, amount)
    );

    count
}

/// Releases the escrowed funds to the beneficiary.
pub fn release_escrow(e: &Env, escrow_id: u32) {
    let mut escrow = get_escrow(e, escrow_id);

    // State & Timelock Validation
    if e.ledger().sequence() < escrow.release_after_ledger {
        panic!("TimelockActive: Cannot release funds before the release_after_ledger");
    }
    if escrow.released || escrow.refunded {
        panic!("InvalidState: Escrow is already settled");
    }

    // Update state
    escrow.released = true;
    e.storage().persistent().set(&DataKey::Escrow(escrow_id), &escrow);

    // Move funds from contract to beneficiary
    spend_balance(e, e.current_contract_address(), escrow.amount);
    receive_balance(e, escrow.beneficiary.clone(), escrow.amount);

    // Emit Event
    e.events().publish(
        (Symbol::new(e, "escrow"), Symbol::new(e, "released"), escrow_id),
        escrow.beneficiary
    );
}

/// Refunds the escrowed funds back to the depositor.
pub fn refund_escrow(e: &Env, escrow_id: u32) {
    let mut escrow = get_escrow(e, escrow_id);

    // State Validation
    if escrow.released || escrow.refunded {
        panic!("InvalidState: Escrow is already settled");
    }

    // Update state
    escrow.refunded = true;
    e.storage().persistent().set(&DataKey::Escrow(escrow_id), &escrow);

    // Move funds from contract back to depositor
    spend_balance(e, e.current_contract_address(), escrow.amount);
    receive_balance(e, escrow.depositor.clone(), escrow.amount);

    // Emit Event
    e.events().publish(
        (Symbol::new(e, "escrow"), Symbol::new(e, "refunded"), escrow_id),
        escrow.depositor
    );
}

/// Helper to read an escrow record
pub fn get_escrow(e: &Env, escrow_id: u32) -> EscrowRecord {
    e.storage()
        .persistent()
        .get(&DataKey::Escrow(escrow_id))
        .expect("Escrow not found")
}

// --- MULTI-RECIPIENT ESCROW LOGIC ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiEscrowRecord {
    pub id: u32,
    pub depositor: Address,
    pub recipients: Vec<SplitRecipient>,
    pub total_amount: i128,
    pub released: bool,
    pub refunded: bool,
}

/// Creates a multi-recipient escrow and locks the funds.
pub fn create_multi_escrow(
    e: &Env,
    depositor: Address,
    recipients: Vec<SplitRecipient>,
    total_amount: i128,
) -> u32 {
    depositor.require_auth();

    // 1. Validate BPS Sums to 10000 (100.00%)
    let mut total_bps: u32 = 0;
    for recipient in recipients.iter() {
        total_bps += recipient.share_bps;
    }
    if total_bps != 10000 {
        panic!("total bps must equal 10000");
    }

    // 2. Move funds from depositor to the contract
    spend_balance(e, depositor.clone(), total_amount);
    receive_balance(e, e.current_contract_address(), total_amount);

    // 3. Manage ID and Storage
    let mut count: u32 = e.storage().instance().get(&DataKey::MultiEscrowCount).unwrap_or(0);
    count += 1;
    e.storage().instance().set(&DataKey::MultiEscrowCount, &count);

    let record = MultiEscrowRecord {
        id: count,
        depositor: depositor.clone(),
        recipients,
        total_amount,
        released: false,
        refunded: false,
    };
    e.storage().persistent().set(&DataKey::MultiEscrow(count), &record);

    // Emit event for observability
    e.events().publish((Symbol::new(e, "multi_escrow"), Symbol::new(e, "created"), count), depositor);

    count
}

/// Releases funds proportionally to all recipients.
pub fn release_multi_escrow(e: &Env, caller: Address, escrow_id: u32) {
    caller.require_auth();

    let mut record: MultiEscrowRecord = e.storage().persistent().get(&DataKey::MultiEscrow(escrow_id)).expect("Escrow not found");

    // 1. Validation: Prevent double-settlement
    if record.released || record.refunded {
        panic!("Already settled");
    }

    // 2. Authorization: Caller must be depositor or admin
    if caller != record.depositor {
        let admin = read_admin(e);
        if caller != admin {
            panic!("unauthorized: must be depositor or admin");
        }
    }

    // 3. Distribute funds proportionally (handling dust)
    let mut remaining_amount = record.total_amount;
    let len = record.recipients.len();

    for (i, recipient) in record.recipients.iter().enumerate() {
        let amount_to_send = if i == (len as usize - 1) {
            remaining_amount // Final recipient gets remainder to prevent dust
        } else {
            (record.total_amount * recipient.share_bps as i128) / 10000
        };

        spend_balance(e, e.current_contract_address(), amount_to_send);
        receive_balance(e, recipient.address.clone(), amount_to_send);
        remaining_amount -= amount_to_send;
    }

    // 4. Update state
    record.released = true;
    e.storage().persistent().set(&DataKey::MultiEscrow(escrow_id), &record);

    e.events().publish((Symbol::new(e, "multi_escrow"), Symbol::new(e, "released"), escrow_id), record.total_amount);
}

/// Refunds the entire amount back to the depositor.
pub fn refund_multi_escrow(e: &Env, caller: Address, escrow_id: u32) {
    caller.require_auth();

    let mut record: MultiEscrowRecord = e.storage().persistent().get(&DataKey::MultiEscrow(escrow_id)).expect("Escrow not found");

    // 1. Validation: Prevent double-settlement
    if record.released || record.refunded {
        panic!("Already settled");
    }

    // 2. Authorization: Caller must be depositor
    if caller != record.depositor {
        panic!("unauthorized: must be depositor");
    }

    // 3. Return funds to depositor
    spend_balance(e, e.current_contract_address(), record.total_amount);
    receive_balance(e, record.depositor.clone(), record.total_amount);

    // 4. Update state
    record.refunded = true;
    e.storage().persistent().set(&DataKey::MultiEscrow(escrow_id), &record);

    e.events().publish((Symbol::new(e, "multi_escrow"), Symbol::new(e, "refunded"), escrow_id), record.depositor);
}