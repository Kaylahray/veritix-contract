#[cfg(test)]
mod escrow_tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};
    use crate::escrow::{EscrowContract, EscrowContractClient}; // Adjust based on your actual trait name

    fn setup_test(e: &Env) -> (Address, Address, EscrowContractClient<'_>) {
        let depositor = Address::generate(e);
        let beneficiary = Address::generate(e);
        let contract_id = e.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(e, &contract_id);
        (depositor, beneficiary, client)
    }

    #[test]
    fn test_create_escrow() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        let amount = 1000i128;

        client.create_escrow(&depositor, &beneficiary, &amount);
        
        let escrow = client.get_escrow(&depositor, &beneficiary);
        assert_eq!(escrow.amount, amount);
        assert_eq!(escrow.released, false);
        assert_eq!(escrow.refunded, false);
    }

    #[test]
    fn test_release_escrow() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        let amount = 1000i128;

        client.create_escrow(&depositor, &beneficiary, &amount);
        client.release_escrow(&beneficiary); // Should be called by beneficiary

        let escrow = client.get_escrow(&depositor, &beneficiary);
        assert!(escrow.released);
        // Verify beneficiary balance increased by 'amount' via your token mock here
    }

    #[test]
    fn test_refund_escrow() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        let amount = 1000i128;

        client.create_escrow(&depositor, &beneficiary, &amount);
        client.refund_escrow(&depositor);

        let escrow = client.get_escrow(&depositor, &beneficiary);
        assert!(escrow.refunded);
    }

    #[test]
    #[should_panic(expected = "not beneficiary")]
    fn test_release_unauthorized_panics() {
        let e = Env::default();
        let (depositor, _, client) = setup_test(&e);
        client.create_escrow(&depositor, &Address::generate(&e), &1000);
        
        // Hacker tries to release
        let hacker = Address::generate(&e);
        client.release_escrow(&hacker);
    }

    #[test]
    #[should_panic(expected = "not depositor")]
    fn test_refund_unauthorized_panics() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        client.create_escrow(&depositor, &beneficiary, &1000);
        
        // Beneficiary tries to refund themselves (unauthorized)
        client.refund_escrow(&beneficiary);
    }

    #[test]
    #[should_panic(expected = "already settled")]
    fn test_double_release_panics() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        client.create_escrow(&depositor, &beneficiary, &1000);
        
        client.release_escrow(&beneficiary);
        client.release_escrow(&beneficiary); // Panic
    }

    #[test]
    #[should_panic(expected = "already settled")]
    fn test_double_refund_panics() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        client.create_escrow(&depositor, &beneficiary, &1000);
        
        client.refund_escrow(&depositor);
        client.refund_escrow(&depositor); // Panic
    }

    #[test]
    #[should_panic(expected = "already settled")]
    fn test_release_after_refund_panics() {
        let e = Env::default();
        let (depositor, beneficiary, client) = setup_test(&e);
        client.create_escrow(&depositor, &beneficiary, &1000);
        
        client.refund_escrow(&depositor);
        client.release_escrow(&beneficiary); // Panic
    }
}