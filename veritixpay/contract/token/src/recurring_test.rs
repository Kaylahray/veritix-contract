#[cfg(test)]
mod recurring_tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};
    use crate::recurring::{RecurringContract, RecurringContractClient}; 

    fn setup_test(e: &Env) -> (Address, Address, RecurringContractClient<'_>) {
        let payer = Address::generate(e);
        let receiver = Address::generate(e);
        let contract_id = e.register_contract(None, RecurringContract);
        let client = RecurringContractClient::new(e, &contract_id);
        
        // Initial ledger setup
        e.ledger().set(soroban_sdk::testutils::LedgerInfo {
            timestamp: 0,
            sequence_number: 100,
            network_id: [0u8; 32],
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 1000,
        });

        (payer, receiver, client)
    }

    #[test]
    fn test_setup_recurring() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        let amount = 500i128;
        let interval = 100u32;

        client.setup_recurring(&payer, &receiver, &amount, &interval);
        
        let record = client.get_recurring(&payer, &receiver);
        assert_eq!(record.amount, amount);
        assert_eq!(record.interval, interval);
        assert!(record.active);
    }

    #[test]
    fn test_execute_recurring() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        let interval = 100u32;
        client.setup_recurring(&payer, &receiver, &500, &interval);

        // Advance ledger: Initial was 100, interval is 100, so 201 is valid
        e.ledger().set_sequence_number(201);
        
        client.execute_recurring(&payer, &receiver);
        
        let record = client.get_recurring(&payer, &receiver);
        assert_eq!(record.last_charged_ledger, 201);
    }

    #[test]
    #[should_panic(expected = "too early")]
    fn test_execute_too_early_panics() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        client.setup_recurring(&payer, &receiver, &500, &100);

        // Only advance by 50 (total 150), which is less than the 100 interval
        e.ledger().set_sequence_number(150);
        client.execute_recurring(&payer, &receiver);
    }

    #[test]
    fn test_cancel_recurring() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        client.setup_recurring(&payer, &receiver, &500, &100);

        client.cancel_recurring(&payer, &receiver);
        
        let record = client.get_recurring(&payer, &receiver);
        assert!(!record.active);
    }

    #[test]
    #[should_panic(expected = "not active")]
    fn test_execute_after_cancel_panics() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        client.setup_recurring(&payer, &receiver, &500, &100);

        client.cancel_recurring(&payer, &receiver);
        
        e.ledger().set_sequence_number(300);
        client.execute_recurring(&payer, &receiver);
    }

    #[test]
    #[should_panic(expected = "unauthorized")]
    fn test_cancel_unauthorized_panics() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        client.setup_recurring(&payer, &receiver, &500, &100);

        let hacker = Address::generate(&e);
        // Only the payer should be able to cancel
        client.cancel_recurring(&hacker, &receiver);
    }

    #[test]
    fn test_multiple_executions() {
        let e = Env::default();
        let (payer, receiver, client) = setup_test(&e);
        client.setup_recurring(&payer, &receiver, &500, &100);

        // Execution 1
        e.ledger().set_sequence_number(201);
        client.execute_recurring(&payer, &receiver);

        // Execution 2
        e.ledger().set_sequence_number(302);
        client.execute_recurring(&payer, &receiver);

        let record = client.get_recurring(&payer, &receiver);
        assert_eq!(record.last_charged_ledger, 302);
    }
}