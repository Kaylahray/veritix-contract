use soroban_sdk::{contracttype, Address, Map, Symbol};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Core token
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
    // Escrow
    EscrowCount,
    Escrow(u32),
    // Recurring payments
    RecurringCount,
    Recurring(u32),
    // Payment splits
    SplitCount,
    Split(u32),
    // Disputes
    DisputeCount,
    Dispute(u32),
    // Payment records
    RecordCount,
    PaymentRecord(u32),
    UserStats(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct EscrowInfo {
    pub sender: Address,
    pub receiver: Address,
    pub amount: i128,
    pub condition: Symbol,
    pub released: bool,
    pub refunded: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct RecurringPayment {
    pub payer: Address,
    pub payee: Address,
    pub amount: i128,
    pub interval: u64,
    pub next_payment: u64,
    pub iterations: u32,
    pub completed: u32,
    pub token_address: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentSplit {
    pub payer: Address,
    pub recipients: Map<Address, i128>,
    pub total_amount: i128,
    pub distributed: bool,
    pub token_address: Address,
}

/// Dispute status is stored as a Symbol: "open" or "resolved".
/// resolver and decision are only meaningful once status == "resolved".
#[derive(Clone)]
#[contracttype]
pub struct Dispute {
    pub payment_id: u32,
    pub initiator: Address,
    pub respondent: Address,
    pub reason: Symbol,
    pub status: Symbol,
    pub resolver: Address,
    pub resolved: bool,
    pub decision: bool,
    pub amount: i128,
    pub token_address: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentRecord {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub timestamp: u64,
    pub token_address: Address,
    pub payment_type: Symbol,
}
