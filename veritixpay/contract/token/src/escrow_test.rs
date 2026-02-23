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

    use crate::splitter::SplitRecipient;
use soroban_sdk::{vec, Vec};

// ... inside your test module ...

#[test]
fn test_create_multi_escrow() {
    let e = Env::default();
    let (depositor, _, client) = setup_test(&e); // Assuming setup_test exists in your test file
    let recipient1 = Address::generate(&e);
    let recipient2 = Address::generate(&e);
    
    let recipients = vec![
        &e,
        SplitRecipient { address: recipient1, share_bps: 6000 },
        SplitRecipient { address: recipient2, share_bps: 4000 },
    ];

    // Assuming you have a wrapper client or call the function directly:
    // create_multi_escrow(&e, depositor.clone(), recipients, 1000);
    // Add assertions for balance deductions and record creation
}

#[test]
fn test_release_multi_escrow_3_recipients() {
    let e = Env::default();
    // Setup environment and balances...
    let depositor = Address::generate(&e);
    let r1 = Address::generate(&e);
    let r2 = Address::generate(&e);
    let r3 = Address::generate(&e);
    
    let recipients = vec![
        &e,
        SplitRecipient { address: r1, share_bps: 5000 },
        SplitRecipient { address: r2, share_bps: 3000 },
        SplitRecipient { address: r3, share_bps: 2000 },
    ];
    
    // Test logic: Create escrow for 1000. Release.
    // Verify balances: r1 = 500, r2 = 300, r3 = 200.
}

#[test]
fn test_refund_multi_escrow() {
    let e = Env::default();
    // Setup environment...
    let depositor = Address::generate(&e);
    let recipients = vec![&e, SplitRecipient { address: Address::generate(&e), share_bps: 10000 }];
    
    // Create escrow for 1000. Refund.
    // Verify depositor gets 1000 back and record is refunded.
}

#[test]
#[should_panic(expected = "total bps must equal 10000")]
fn test_invalid_bps_panics() {
    let e = Env::default();
    let depositor = Address::generate(&e);
    let recipients = vec![
        &e,
        SplitRecipient { address: Address::generate(&e), share_bps: 9999 }
    ];

    crate::escrow::create_multi_escrow(&e, depositor, recipients, 1000);
}
}