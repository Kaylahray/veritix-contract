use std::collections::HashMap;

// --- 1. Core State Models ---
// In your actual app, these would likely be imported from an `escrow.rs` module.

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EscrowState {
    Funded,
    Disputed,
    Released,
    Resolved,
}

#[derive(Debug, PartialEq)]
pub enum ResolutionOutcome {
    Beneficiary,
    Depositor,
}

pub struct Escrow {
    pub id: u64,
    pub depositor: String,
    pub beneficiary: String,
    pub resolver: String,
    pub state: EscrowState,
    pub amount: u64,
}

pub struct Dispute {
    pub initiator: String,
}

// --- 2. State Ledger (Mock Environment) ---
// This simulates your smart contract's state storage.

pub struct EscrowEnvironment {
    pub escrows: HashMap<u64, Escrow>,
    pub disputes: HashMap<u64, Dispute>,
    pub balances: HashMap<String, u64>,
}

impl EscrowEnvironment {
    pub fn new() -> Self {
        Self {
            escrows: HashMap::new(),
            disputes: HashMap::new(),
            balances: HashMap::new(),
        }
    }

    // --- 3. The Required Dispute Logic ---

    pub fn open_dispute(&mut self, escrow_id: u64, caller: &str) {
        let escrow = self.escrows.get_mut(&escrow_id).expect("Escrow not found");
        
        // State checks
        if escrow.state == EscrowState::Released {
            panic!("InvalidState: Cannot open dispute on a released escrow");
        }
        if escrow.state == EscrowState::Resolved {
            panic!("InvalidState: Cannot open dispute on a resolved escrow");
        }
        if escrow.state == EscrowState::Disputed {
            panic!("InvalidState: Escrow is already disputed");
        }

        // Authorization check
        if caller != escrow.depositor && caller != escrow.beneficiary {
            panic!("Unauthorized: Only depositor or beneficiary can open a dispute");
        }

        // Update state and store record
        escrow.state = EscrowState::Disputed;
        self.disputes.insert(escrow_id, Dispute { initiator: caller.to_string() });
    }

    pub fn resolve_dispute(&mut self, escrow_id: u64, caller: &str, outcome: ResolutionOutcome) {
        let escrow = self.escrows.get_mut(&escrow_id).expect("Escrow not found");

        // State checks
        if escrow.state == EscrowState::Resolved {
            panic!("AlreadyResolved: This dispute has already been resolved");
        }
        if escrow.state != EscrowState::Disputed {
            panic!("InvalidState: Escrow is not currently disputed");
        }

        // Authorization check
        if caller != escrow.resolver {
            panic!("UnauthorizedResolver: Only the designated resolver can resolve this");
        }

        let amount = escrow.amount;
        let target = match outcome {
            ResolutionOutcome::Beneficiary => escrow.beneficiary.clone(),
            ResolutionOutcome::Depositor => escrow.depositor.clone(),
        };

        // Transfer funds and update state
        *self.balances.entry(target).or_insert(0) += amount;
        escrow.state = EscrowState::Resolved;
    }

    // Helper for testing the released state panic
    pub fn release_escrow(&mut self, escrow_id: u64) {
        let escrow = self.escrows.get_mut(&escrow_id).expect("Escrow not found");
        escrow.state = EscrowState::Released;
    }
}

// --- 4. Unit Tests ---

#[cfg(test)]
mod dispute_tests {
    use super::*;

    fn setup_environment() -> (EscrowEnvironment, u64, String, String, String) {
        let mut env = EscrowEnvironment::new();
        let escrow_id = 1;
        let depositor = "Alice".to_string();
        let beneficiary = "Bob".to_string();
        let resolver = "Charlie".to_string();

        env.escrows.insert(escrow_id, Escrow {
            id: escrow_id,
            depositor: depositor.clone(),
            beneficiary: beneficiary.clone(),
            resolver: resolver.clone(),
            state: EscrowState::Funded,
            amount: 100,
        });

        // Initialize balances
        env.balances.insert(depositor.clone(), 0);
        env.balances.insert(beneficiary.clone(), 0);

        (env, escrow_id, depositor, beneficiary, resolver)
    }

    // 1. test_open_dispute
    #[test]
    fn test_open_dispute() {
        let (mut env, escrow_id, depositor, _, _) = setup_environment();

        env.open_dispute(escrow_id, &depositor);

        let dispute = env.disputes.get(&escrow_id).expect("Dispute record missing");
        assert_eq!(dispute.initiator, depositor);
        
        let escrow = env.escrows.get(&escrow_id).unwrap();
        assert_eq!(escrow.state, EscrowState::Disputed);
    }

    // 2. test_resolve_for_beneficiary
    #[test]
    fn test_resolve_for_beneficiary() {
        let (mut env, escrow_id, depositor, beneficiary, resolver) = setup_environment();
        env.open_dispute(escrow_id, &depositor);

        env.resolve_dispute(escrow_id, &resolver, ResolutionOutcome::Beneficiary);

        assert_eq!(*env.balances.get(&beneficiary).unwrap(), 100);
        let escrow = env.escrows.get(&escrow_id).unwrap();
        assert_eq!(escrow.state, EscrowState::Resolved);
    }

    // 3. test_resolve_for_depositor
    #[test]
    fn test_resolve_for_depositor() {
        let (mut env, escrow_id, depositor, _, resolver) = setup_environment();
        env.open_dispute(escrow_id, &depositor);

        env.resolve_dispute(escrow_id, &resolver, ResolutionOutcome::Depositor);

        assert_eq!(*env.balances.get(&depositor).unwrap(), 100);
        let escrow = env.escrows.get(&escrow_id).unwrap();
        assert_eq!(escrow.state, EscrowState::Resolved);
    }

    // 4. test_open_dispute_unauthorized_panics
    #[test]
    #[should_panic(expected = "Unauthorized: Only depositor or beneficiary can open a dispute")]
    fn test_open_dispute_unauthorized_panics() {
        let (mut env, escrow_id, _, _, _) = setup_environment();
        env.open_dispute(escrow_id, "Eve"); // Eve is an external party
    }

    // 5. test_resolve_unauthorized_panics
    #[test]
    #[should_panic(expected = "UnauthorizedResolver: Only the designated resolver can resolve this")]
    fn test_resolve_unauthorized_panics() {
        let (mut env, escrow_id, depositor, _, _) = setup_environment();
        env.open_dispute(escrow_id, &depositor);
        
        env.resolve_dispute(escrow_id, "Eve", ResolutionOutcome::Depositor);
    }

    // 6. test_double_resolve_panics
    #[test]
    #[should_panic(expected = "AlreadyResolved: This dispute has already been resolved")]
    fn test_double_resolve_panics() {
        let (mut env, escrow_id, depositor, _, resolver) = setup_environment();
        env.open_dispute(escrow_id, &depositor);

        env.resolve_dispute(escrow_id, &resolver, ResolutionOutcome::Beneficiary);
        env.resolve_dispute(escrow_id, &resolver, ResolutionOutcome::Depositor); // Should panic here
    }

    // 7. test_dispute_already_released_escrow_panics
    #[test]
    #[should_panic(expected = "InvalidState: Cannot open dispute on a released escrow")]
    fn test_dispute_already_released_escrow_panics() {
        let (mut env, escrow_id, depositor, _, _) = setup_environment();
        
        env.release_escrow(escrow_id); // Change state to released
        env.open_dispute(escrow_id, &depositor); // Should panic here
    }
}