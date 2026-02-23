#[cfg(test)]
mod splitter_tests {
    use super::*;
    // Replace with your actual environment imports (e.g., soroban_sdk or cosmwasm_std)
    use crate::{Contract, Recipient}; 

    #[test]
    fn test_create_split() {
        let env = setup_env();
        let sender = env.address("sender");
        let total_amount = 10_000u128;

        // Verify record is stored and initial state is correct
        let split_id = create_split(&env, &sender, total_amount);
        let split = get_split(&env, split_id);
        
        assert_eq!(split.sender, sender);
        assert_eq!(split.amount, total_amount);
        // Add check for sender balance deduction here based on your ledger implementation
    }

    #[test]
    fn test_distribute_two_recipients() {
        let env = setup_env();
        let recipients = vec![
            Recipient { addr: env.address("u1"), bps: 5000 },
            Recipient { addr: env.address("u2"), bps: 5000 },
        ];
        
        let results = calculate_distribution(1000, &recipients);
        assert_eq!(results[0].amount, 500);
        assert_eq!(results[1].amount, 500);
    }

    #[test]
    fn test_distribute_three_recipients() {
        let env = setup_env();
        let recipients = vec![
            Recipient { addr: env.address("u1"), bps: 5000 },
            Recipient { addr: env.address("u2"), bps: 3000 },
            Recipient { addr: env.address("u3"), bps: 2000 },
        ];
        
        let results = calculate_distribution(1000, &recipients);
        assert_eq!(results[0].amount, 500);
        assert_eq!(results[1].amount, 300);
        assert_eq!(results[2].amount, 200);
    }

    #[test]
    #[should_panic(expected = "BPS_SUM_MUST_BE_10000")]
    fn test_invalid_bps_panics() {
        let recipients = vec![Recipient { addr: "u1", bps: 9999 }];
        validate_split_config(&recipients);
    }

    #[test]
    #[should_panic(expected = "ALREADY_DISTRIBUTED")]
    fn test_double_distribute_panics() {
        let mut split = setup_active_split();
        distribute(&mut split); // First call
        distribute(&mut split); // Should panic
    }

    #[test]
    #[should_panic(expected = "UNAUTHORIZED")]
    fn test_distribute_unauthorized_panics() {
        let env = setup_env();
        let hacker = env.address("hacker");
        distribute_as(&env, hacker, split_id);
    }

    #[test]
    fn test_distribute_rounds_correctly() {
        let env = setup_env();
        // Case: 10 units split between 3 people (3333, 3333, 3334 BPS)
        let recipients = vec![
            Recipient { addr: env.address("u1"), bps: 3333 },
            Recipient { addr: env.address("u2"), bps: 3333 },
            Recipient { addr: env.address("u3"), bps: 3334 },
        ];

        let total = 10u128;
        let shares = calculate_distribution(total, &recipients);
        
        let sum: u128 = shares.iter().map(|s| s.amount).sum();
        
        // Mathematically: (3.333) + (3.333) + (3.334) = 10.0
        // In integer math: 3 + 3 + 3 = 9. 
        // We must ensure the sum equals the total.
        assert_eq!(sum, total, "Rounding error: Dust remaining in contract");
        assert_eq!(shares[0].amount, 3);
        assert_eq!(shares[1].amount, 3);
        assert_eq!(shares[2].amount, 4); // Last recipient picks up the remainder
    }
}