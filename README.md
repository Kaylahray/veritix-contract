# Veritix Contracts

On-chain payment infrastructure for the Veritix ticketing platform, built with **Rust** and **Soroban** (Stellar's smart contract platform).

This repository implements a token contract with advanced payment primitives — escrow, recurring payments, payment splitting, and dispute resolution — designed to power trustless financial operations on the Stellar network.

---

## What's in this repo

This is a single Soroban smart contract package located at `veritixpay/contract/token/`. It contains:

| Module | File | Description |
|--------|------|-------------|
| Token core | `contract.rs` | Standard token — mint, burn, transfer, approve |
| Admin | `admin.rs` | Administrator address management |
| Balances | `balance.rs` | Persistent account balance tracking |
| Allowances | `allowance.rs` | Delegated spending with ledger-based expiration |
| Metadata | `metadata.rs` | Token name, symbol, and decimals |
| Storage | `storage_types.rs` | Shared data structures and storage key definitions |
| Escrow | `escrow.rs` | Conditional fund holding and release |
| Recurring payments | `recurring.rs` | Automated periodic payment scheduling |
| Payment splitting | `splitter.rs` | Distribute a payment among multiple recipients |
| Dispute resolution | `dispute.rs` | Third-party arbitration for contested payments |
| Tests | `test.rs` | Unit tests for the token core |

---

## Repository layout

```
veritix-contract/
├── veritixpay/
│   ├── Cargo.toml              # Workspace config (soroban-sdk 20.5.0)
│   └── contract/
│       └── token/
│           ├── Cargo.toml      # Package: veritix-token
│           ├── Makefile        # build / test / fmt shortcuts
│           └── src/
│               ├── lib.rs
│               ├── contract.rs
│               ├── admin.rs
│               ├── allowance.rs
│               ├── balance.rs
│               ├── metadata.rs
│               ├── storage_types.rs
│               ├── escrow.rs
│               ├── recurring.rs
│               ├── splitter.rs
│               ├── dispute.rs
│               └── test.rs
├── .gitignore
├── CONTRIBUTING.md
└── README.md
```

---

## Why Stellar & Soroban

- Deterministic execution and predictable fees
- Fast finality suitable for real-time ticket validation
- Rust's safety guarantees at the contract level
- Native integration with the Stellar ecosystem

---

## Payment modules

### Escrow (`escrow.rs`)
Holds funds in the contract until a condition is met. Sender can trigger a refund; receiver gets funds only when the condition passes.

```
create(sender, receiver, amount, condition, token) → escrow_id
release(escrow_id, token)   // condition must be true
refund(escrow_id, token)    // sender reclaims funds
```

### Recurring payments (`recurring.rs`)
Schedule periodic transfers between two parties. First payment executes immediately at setup; subsequent payments are gated by ledger-based interval checks.

```
setup(payer, payee, amount, interval, iterations, token) → payment_id
execute(payment_id)
```

### Payment splitting (`splitter.rs`)
Split a single payment across multiple recipients by percentage. Percentages must sum to 100.

```
create_split(payer, recipients, total_amount, token) → split_id
distribute(split_id)
```

### Dispute resolution (`dispute.rs`)
Open a dispute on a payment and assign a third-party resolver. The resolver's decision determines who receives the funds.

```
open_dispute(payment_id, initiator, respondent, reason, amount, token) → dispute_id
resolve_dispute(dispute_id, resolver, decision)
  // decision=true  → funds returned to initiator
  // decision=false → funds sent to respondent
```

---

## Getting started

### Requirements

- [Rust](https://rustup.rs/) (stable, with `wasm32-unknown-unknown` target)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install stellar-cli
```

### Build

```bash
cd veritixpay/contract/token
make build
# or: stellar contract build
```

### Test

```bash
make test
# or: cargo test
```

### Format

```bash
make fmt
```

---

## Deploying to Stellar testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/veritix_token.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

---

## Storage design

Soroban storage is key-value. This contract uses three tiers:

| Tier | Used for | TTL |
|------|----------|-----|
| Instance | Admin, token metadata | 7 days (auto-bumped) |
| Persistent | Balances, escrows, payments | 30 days (auto-bumped) |
| Temporary | Allowances (with ledger expiry) | Per allowance expiration |

All TTLs are bumped on access to keep live data from expiring.

---

## Security notes

- Every mutable operation calls `require_auth()` on the relevant signer
- Amounts are validated as non-negative on all entry points
- Escrow, recurring, and split records use a `released` / `distributed` flag to prevent double execution
- Decimals are capped at 18

This contract has not been audited. Do not deploy to mainnet without a full audit.

---

## Related repositories

- **Backend:** https://github.com/Lead-Studios/veritix-backend
- **Web client:** https://github.com/Lead-Studios/veritix-web

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

## License

MIT
