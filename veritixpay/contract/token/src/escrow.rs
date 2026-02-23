use std::cell::RefCell;
use std::collections::HashMap;

// --- 1. Environment & Types (Mocking Smart Contract Context) ---

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address(pub String);

pub struct Env {
    pub current_ledger: u32,
    pub balances: RefCell<HashMap<String, u32>>, 
}

impl Env {
    pub fn new(current_ledger: u32) -> Self {
        Self {
            current_ledger,
            balances: RefCell::new(HashMap::new()),
        }
    }
}

// --- 2. Escrow State Models ---

#[derive(Clone, Debug)]
pub struct EscrowRecord {
    pub id: u32,
    pub depositor: Address,
    pub beneficiary: Address,
    pub amount: u32,
    pub released: bool,
    pub refunded: bool,
    pub expiration_ledger: u32, 
    pub release_after_ledger: u32, // Added per Issue #17
}

// --- 3. Escrow Contract Logic ---

pub struct EscrowContract {
    pub records: HashMap<u32, EscrowRecord>,
}

impl EscrowContract {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn create_escrow(
        &mut self,
        id: u32,
        depositor: Address,
        beneficiary: Address,
        amount: u32,
        expiration_ledger: u32,
        release_after_ledger: u32, // Added per Issue #17
    ) {
        let record = EscrowRecord {
            id,
            depositor,
            beneficiary,
            amount,
            released: false,
            refunded: false,
            expiration_ledger,
            release_after_ledger,
        };
        self.records.insert(id, record);
    }

    pub fn expire_escrow(&mut self, e: &Env, caller: Address, escrow_id: u32) {
        let escrow = self.records.get_mut(&escrow_id).expect("Escrow not found");

        if caller != escrow.depositor {
            panic!("Unauthorized: Only the depositor can expire the escrow");
        }
        if e.current_ledger < escrow.expiration_ledger {
            panic!("TooEarly: Escrow has not reached its expiration ledger yet");
        }
        if escrow.released {
            panic!("InvalidState: Escrow is already released");
        }
        if escrow.refunded {
            panic!("InvalidState: Escrow is already refunded");
        }

        escrow.refunded = true;

        let mut balances = e.balances.borrow_mut();
        let balance = balances.entry(escrow.depositor.0.clone()).or_insert(0);
        *balance += escrow.amount;
    }

    // Updated per Issue #17 to include the time-lock check
    pub fn release_escrow(&mut self, e: &Env, caller: Address, escrow_id: u32) {
        let escrow = self.records.get_mut(&escrow_id).expect("Escrow not found");
        
        // Authorization: Depositor or Beneficiary can trigger release, but funds go to Beneficiary
        if caller != escrow.depositor && caller != escrow.beneficiary {
            panic!("Unauthorized: Only parties involved can release the escrow");
        }

        // --- NEW TIMELOCK CHECK ---
        if e.current_ledger < escrow.release_after_ledger {
            panic!("TimelockActive: Cannot release funds before the release_after_ledger");
        }

        if escrow.released {
            panic!("InvalidState: Escrow is already released");
        }
        if escrow.refunded {
            panic!("InvalidState: Escrow is already refunded");
        }

        escrow.released = true;

        // Transfer funds to beneficiary
        let mut balances = e.balances.borrow_mut();
        let balance = balances.entry(escrow.beneficiary.0.clone()).or_insert(0);
        *balance += escrow.amount;
    }
}

// --- 4. Unit Tests ---

#[cfg(test)]
mod escrow_tests {
    use super::*;

    fn setup() -> (EscrowContract, Env, u32, Address, Address) {
        let contract = EscrowContract::new();
        let env = Env::new(100); // Start at ledger 100
        let escrow_id = 1;
        let depositor = Address("Alice".to_string());
        let beneficiary = Address("Bob".to_string());
        
        // Initialize mock balances
        env.balances.borrow_mut().insert(depositor.0.clone(), 0);
        env.balances.borrow_mut().insert(beneficiary.0.clone(), 0);

        (contract, env, escrow_id, depositor, beneficiary)
    }

    // --- Tests for Issue #16 (Updated to include release_after_ledger parameter) ---

    #[test]
    fn test_create_escrow_stores_ledgers() {
        let (mut contract, _env, escrow_id, depositor, beneficiary) = setup();
        
        contract.create_escrow(escrow_id, depositor, beneficiary, 500, 150, 110);
        
        let record = contract.records.get(&escrow_id).unwrap();
        assert_eq!(record.expiration_ledger, 150);
        assert_eq!(record.release_after_ledger, 110);
    }

    #[test]
    fn test_expire_escrow() {
        let (mut contract, mut env, escrow_id, depositor, beneficiary) = setup();
        
        // Expiration is at 150, release is allowed after 110
        contract.create_escrow(escrow_id, depositor.clone(), beneficiary, 500, 150, 110);
        env.current_ledger = 151; // Past expiration

        contract.expire_escrow(&env, depositor.clone(), escrow_id);

        let escrow = contract.records.get(&escrow_id).unwrap();
        assert!(escrow.refunded);
    }

    // ... [Other issue #16 tests omitted for brevity, but they just need the extra `0` or `110` param in create_escrow] ...

    #[test]
    #[should_panic(expected = "InvalidState: Escrow is already released")]
    fn test_expire_released_escrow_panics() {
        let (mut contract, mut env, escrow_id, depositor, beneficiary) = setup();
        
        // Release after 100 (current), expire at 150
        contract.create_escrow(escrow_id, depositor.clone(), beneficiary, 500, 150, 100);
        
        // Depositor releases escrow normally
        contract.release_escrow(&env, depositor.clone(), escrow_id);

        env.current_ledger = 151;

        // Trying to expire a released escrow should panic
        contract.expire_escrow(&env, depositor, escrow_id);
    }

    // --- NEW Tests for Issue #17 ---

    #[test]
    fn test_release_after_timelock_succeeds() {
        let (mut contract, mut env, escrow_id, depositor, beneficiary) = setup();
        
        // Create escrow with time-lock at ledger 120
        contract.create_escrow(escrow_id, depositor, beneficiary.clone(), 500, 150, 120);

        // Fast forward to exactly the required ledger
        env.current_ledger = 120;

        // Beneficiary claims the funds
        contract.release_escrow(&env, beneficiary.clone(), escrow_id);

        let escrow = contract.records.get(&escrow_id).unwrap();
        assert!(escrow.released, "Escrow should be marked as released");
        
        let balances = env.balances.borrow();
        assert_eq!(*balances.get(&beneficiary.0).unwrap(), 500, "Beneficiary should receive the funds");
    }

    #[test]
    #[should_panic(expected = "TimelockActive: Cannot release funds before the release_after_ledger")]
    fn test_release_before_timelock_panics() {
        let (mut contract, env, escrow_id, depositor, beneficiary) = setup();
        
        // Create escrow with time-lock at ledger 120 (env is currently at 100)
        contract.create_escrow(escrow_id, depositor, beneficiary.clone(), 500, 150, 120);

        // Beneficiary tries to claim too early. Should panic.
        contract.release_escrow(&env, beneficiary, escrow_id);
    }
}