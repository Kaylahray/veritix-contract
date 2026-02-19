use soroban_sdk::{Env, Symbol, Address};
use soroban_sdk::token::Client as TokenClient;
use crate::storage_types::{DataKey, EscrowInfo, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};

pub struct EscrowContract;

impl EscrowContract {
    /// Lock `amount` tokens from `sender` into escrow pending a condition.
    /// Returns the new escrow ID.
    pub fn create(
        env: Env,
        sender: Address,
        receiver: Address,
        amount: i128,
        condition: Symbol,
        token_address: Address,
    ) -> u32 {
        sender.require_auth();

        let token_client = TokenClient::new(&env, &token_address);
        let sender_balance = token_client.balance(&sender);
        assert!(sender_balance >= amount, "Insufficient balance");

        // Transfer tokens from sender into this contract
        token_client.transfer(&sender, &env.current_contract_address(), &amount);

        let escrow_id: u32 = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::EscrowCount)
            .unwrap_or(0)
            + 1;
        env.storage()
            .persistent()
            .set(&DataKey::EscrowCount, &escrow_id);

        let escrow_info = EscrowInfo {
            sender,
            receiver,
            amount,
            condition,
            released: false,
            refunded: false,
        };
        let key = DataKey::Escrow(escrow_id);
        env.storage().persistent().set(&key, &escrow_info);
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        escrow_id
    }

    /// Release escrowed funds to the receiver once the condition is satisfied.
    pub fn release(env: Env, escrow_id: u32, token_address: Address) {
        let key = DataKey::Escrow(escrow_id);
        let mut escrow_info: EscrowInfo = env
            .storage()
            .persistent()
            .get::<DataKey, EscrowInfo>(&key)
            .expect("escrow not found");

        assert!(!escrow_info.released, "Already released");
        assert!(!escrow_info.refunded, "Already refunded");

        let token_client = TokenClient::new(&env, &token_address);
        token_client.transfer(
            &env.current_contract_address(),
            &escrow_info.receiver,
            &escrow_info.amount,
        );

        escrow_info.released = true;
        env.storage().persistent().set(&key, &escrow_info);
    }

    /// Refund escrowed funds back to the sender. Requires sender authorization.
    pub fn refund(env: Env, escrow_id: u32, token_address: Address) {
        let key = DataKey::Escrow(escrow_id);
        let mut escrow_info: EscrowInfo = env
            .storage()
            .persistent()
            .get::<DataKey, EscrowInfo>(&key)
            .expect("escrow not found");

        escrow_info.sender.require_auth();
        assert!(!escrow_info.released, "Already released");
        assert!(!escrow_info.refunded, "Already refunded");

        let token_client = TokenClient::new(&env, &token_address);
        token_client.transfer(
            &env.current_contract_address(),
            &escrow_info.sender,
            &escrow_info.amount,
        );

        escrow_info.refunded = true;
        env.storage().persistent().set(&key, &escrow_info);
    }

    /// Retrieve escrow details by ID.
    pub fn get_escrow(env: Env, escrow_id: u32) -> EscrowInfo {
        env.storage()
            .persistent()
            .get::<DataKey, EscrowInfo>(&DataKey::Escrow(escrow_id))
            .expect("escrow not found")
    }
}
