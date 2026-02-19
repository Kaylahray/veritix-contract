use soroban_sdk::{Env, Address, Map};
use soroban_sdk::token::Client as TokenClient;
use crate::storage_types::{DataKey, PaymentSplit, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};

pub struct PaymentSplitter;

impl PaymentSplitter {
    /// Create a payment split record. `recipients` maps each recipient address
    /// to their share as a percentage. Percentages must sum to 100.
    /// Returns the split ID.
    pub fn create_split(
        env: Env,
        payer: Address,
        recipients: Map<Address, i128>,
        total_amount: i128,
        token_address: Address,
    ) -> u32 {
        payer.require_auth();

        let mut total_percent: i128 = 0;
        for (_, percent) in recipients.iter() {
            total_percent += percent;
        }
        assert!(total_percent == 100, "Percentages must sum to 100");

        let token_client = TokenClient::new(&env, &token_address);
        let payer_balance = token_client.balance(&payer);
        assert!(payer_balance >= total_amount, "Insufficient balance");

        let split_id: u32 = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::SplitCount)
            .unwrap_or(0)
            + 1;
        env.storage()
            .persistent()
            .set(&DataKey::SplitCount, &split_id);

        let split_info = PaymentSplit {
            payer,
            recipients,
            total_amount,
            distributed: false,
            token_address,
        };
        let key = DataKey::Split(split_id);
        env.storage().persistent().set(&key, &split_info);
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        split_id
    }

    /// Distribute funds to all recipients according to their percentages.
    /// Requires payer authorization. Can only be called once per split.
    pub fn distribute(env: Env, split_id: u32) {
        let key = DataKey::Split(split_id);
        let mut split_info: PaymentSplit = env
            .storage()
            .persistent()
            .get::<DataKey, PaymentSplit>(&key)
            .expect("split not found");

        split_info.payer.require_auth();
        assert!(!split_info.distributed, "Already distributed");

        let token_client = TokenClient::new(&env, &split_info.token_address);

        for (recipient, percent) in split_info.recipients.iter() {
            let amount = split_info.total_amount * percent / 100;
            token_client.transfer(&split_info.payer, &recipient, &amount);
        }

        split_info.distributed = true;
        env.storage().persistent().set(&key, &split_info);
    }

    /// Retrieve split details by ID.
    pub fn get_split(env: Env, split_id: u32) -> PaymentSplit {
        env.storage()
            .persistent()
            .get::<DataKey, PaymentSplit>(&DataKey::Split(split_id))
            .expect("split not found")
    }
}
