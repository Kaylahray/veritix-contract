use soroban_sdk::{Env, Address, Symbol};
use soroban_sdk::token::Client as TokenClient;
use crate::storage_types::{DataKey, Dispute, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};

pub struct DisputeResolution;

impl DisputeResolution {
    /// Open a dispute for a payment. Requires initiator authorization.
    /// Returns the dispute ID.
    pub fn open_dispute(
        env: Env,
        payment_id: u32,
        initiator: Address,
        respondent: Address,
        reason: Symbol,
        amount: i128,
        token_address: Address,
    ) -> u32 {
        initiator.require_auth();

        let dispute_id: u32 = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::DisputeCount)
            .unwrap_or(0)
            + 1;
        env.storage()
            .persistent()
            .set(&DataKey::DisputeCount, &dispute_id);

        // Use initiator as a placeholder resolver until one is assigned via resolve_dispute
        let dispute_info = Dispute {
            payment_id,
            initiator: initiator.clone(),
            respondent,
            reason,
            status: Symbol::new(&env, "open"),
            resolver: initiator,
            resolved: false,
            decision: false,
            amount,
            token_address,
        };
        let key = DataKey::Dispute(dispute_id);
        env.storage().persistent().set(&key, &dispute_info);
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        dispute_id
    }

    /// Resolve a dispute. Requires resolver authorization.
    ///
    /// `decision == true`  → funds returned to initiator (payer)
    /// `decision == false` → funds sent to respondent (payee)
    pub fn resolve_dispute(
        env: Env,
        dispute_id: u32,
        resolver: Address,
        decision: bool,
    ) {
        resolver.require_auth();

        let key = DataKey::Dispute(dispute_id);
        let mut dispute_info: Dispute = env
            .storage()
            .persistent()
            .get::<DataKey, Dispute>(&key)
            .expect("dispute not found");

        assert!(
            dispute_info.status == Symbol::new(&env, "open"),
            "Dispute not open"
        );

        let token_client = TokenClient::new(&env, &dispute_info.token_address);

        if decision {
            // Return funds to payer (initiator)
            token_client.transfer(
                &env.current_contract_address(),
                &dispute_info.initiator,
                &dispute_info.amount,
            );
        } else {
            // Send funds to payee (respondent)
            token_client.transfer(
                &env.current_contract_address(),
                &dispute_info.respondent,
                &dispute_info.amount,
            );
        }

        dispute_info.status = Symbol::new(&env, "resolved");
        dispute_info.resolver = resolver;
        dispute_info.resolved = true;
        dispute_info.decision = decision;
        env.storage().persistent().set(&key, &dispute_info);
    }

    /// Retrieve dispute details by ID.
    pub fn get_dispute(env: Env, dispute_id: u32) -> Dispute {
        env.storage()
            .persistent()
            .get::<DataKey, Dispute>(&DataKey::Dispute(dispute_id))
            .expect("dispute not found")
    }
}
