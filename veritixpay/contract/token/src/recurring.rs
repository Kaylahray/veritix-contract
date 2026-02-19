use soroban_sdk::{Env, Address};
use soroban_sdk::token::Client as TokenClient;
use crate::storage_types::{DataKey, RecurringPayment, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};

pub struct RecurringPayments;

impl RecurringPayments {
    /// Schedule a recurring payment. Executes the first payment immediately.
    /// Returns the payment ID.
    pub fn setup(
        env: Env,
        payer: Address,
        payee: Address,
        amount: i128,
        interval: u64,
        iterations: u32,
        token_address: Address,
    ) -> u32 {
        payer.require_auth();

        let token_client = TokenClient::new(&env, &token_address);
        let payer_balance = token_client.balance(&payer);
        assert!(payer_balance >= amount, "Insufficient balance");

        let payment_id: u32 = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::RecurringCount)
            .unwrap_or(0)
            + 1;
        env.storage()
            .persistent()
            .set(&DataKey::RecurringCount, &payment_id);

        let payment_info = RecurringPayment {
            payer: payer.clone(),
            payee,
            amount,
            interval,
            next_payment: env.ledger().timestamp() + interval,
            iterations,
            completed: 0,
            token_address,
        };
        let key = DataKey::Recurring(payment_id);
        env.storage().persistent().set(&key, &payment_info);
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        // Execute the first payment immediately
        Self::execute_payment(&env, payment_id);

        payment_id
    }

    /// Execute the next scheduled payment for an active recurring payment.
    pub fn execute(env: Env, payment_id: u32) {
        let payment_info: RecurringPayment = env
            .storage()
            .persistent()
            .get::<DataKey, RecurringPayment>(&DataKey::Recurring(payment_id))
            .expect("payment not found");

        assert!(
            env.ledger().timestamp() >= payment_info.next_payment,
            "Too early for next payment"
        );
        assert!(
            payment_info.completed < payment_info.iterations,
            "All payments completed"
        );

        Self::execute_payment(&env, payment_id);
    }

    fn execute_payment(env: &Env, payment_id: u32) {
        let mut payment_info: RecurringPayment = env
            .storage()
            .persistent()
            .get::<DataKey, RecurringPayment>(&DataKey::Recurring(payment_id))
            .expect("payment not found");

        let token_client = TokenClient::new(env, &payment_info.token_address);
        token_client.transfer(
            &payment_info.payer,
            &payment_info.payee,
            &payment_info.amount,
        );

        payment_info.completed += 1;
        payment_info.next_payment = env.ledger().timestamp() + payment_info.interval;

        let key = DataKey::Recurring(payment_id);
        env.storage().persistent().set(&key, &payment_info);
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    }

    /// Retrieve payment details by ID.
    pub fn get_payment(env: Env, payment_id: u32) -> RecurringPayment {
        env.storage()
            .persistent()
            .get::<DataKey, RecurringPayment>(&DataKey::Recurring(payment_id))
            .expect("payment not found")
    }
}
