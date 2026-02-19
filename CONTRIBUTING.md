# Contributing to Veritix Contracts

This is an open source project and contributions are welcome. This guide explains how to get the code running locally, how the codebase is structured, and what to keep in mind when submitting changes.

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust (stable) | latest stable | https://rustup.rs |
| wasm32 target | — | `rustup target add wasm32-unknown-unknown` |
| Stellar CLI | latest | `cargo install stellar-cli` |

Verify your setup:

```bash
rustc --version
stellar --version
```

---

## Getting the code

```bash
git clone https://github.com/Lead-Studios/veritix-contract.git
cd veritix-contract/veritixpay/contract/token
```

---

## Building and testing

All commands are available via the `Makefile` inside `veritixpay/contract/token/`:

```bash
make build   # compiles to WASM
make test    # runs the full test suite
make fmt     # formats Rust code with rustfmt
make clean   # removes build artifacts
```

You can also use Cargo directly:

```bash
cargo test
cargo build --target wasm32-unknown-unknown --release
```

**All tests must pass before submitting a pull request.**

---

## Project structure

The contract lives entirely in `veritixpay/contract/token/src/`. Each file is a focused module:

```
src/
├── lib.rs            # Module declarations and public exports
├── contract.rs       # Main token interface (mint, burn, transfer, approve)
├── admin.rs          # Admin address read/write helpers
├── allowance.rs      # Allowance logic with ledger-based expiration
├── balance.rs        # Persistent balance management
├── metadata.rs       # Token name, symbol, decimals
├── storage_types.rs  # All DataKey variants and shared structs
├── escrow.rs         # Escrow: create / release / refund
├── recurring.rs      # Recurring payments: setup / execute
├── splitter.rs       # Payment splitting: create_split / distribute
├── dispute.rs        # Dispute resolution: open / resolve
└── test.rs           # Unit tests (token core)
```

A new feature should live in its own module file and be declared in `lib.rs`.

---

## How to add a new module

1. Create `src/your_module.rs`
2. Add `pub mod your_module;` to `src/lib.rs`
3. Add any new `DataKey` variants to `storage_types.rs`
4. Write tests — either in a `#[cfg(test)]` block inside your module, or add cases to `test.rs`
5. Run `make test` and `make fmt`

---

## Writing tests

Tests use the Soroban test environment. Look at `test.rs` for examples of how to:

- Create a test environment with `Env::default()`
- Register and call the contract via `TokenClient`
- Mock authorization with `mock_all_auths()`

```rust
#[test]
fn test_your_feature() {
    let env = Env::default();
    env.mock_all_auths();
    // ... register contract, call functions, assert state
}
```

Run only your tests:

```bash
cargo test your_feature
```

---

## Storage conventions

- Use `DataKey` variants from `storage_types.rs` — don't invent inline storage keys
- Persist records with `env.storage().persistent()` and bump TTL using `BALANCE_BUMP_AMOUNT`
- Use `env.storage().instance()` for contract-wide config (admin, metadata)
- Every new record type that needs a counter follows the `*Count` / `*(u32)` pattern already used by `EscrowCount`/`Escrow(u32)`

---

## Authorization

Every function that mutates state on behalf of a user must call `address.require_auth()` (or `require_auth_for_args`) before touching storage. Do not skip this.

---

## Pull request checklist

- [ ] `make test` passes with no failures
- [ ] `make fmt` has been run (no formatting diffs)
- [ ] New logic has at least one test covering the happy path
- [ ] Error/panic paths are tested where practical
- [ ] No new `unwrap()` calls on external/untrusted data
- [ ] `storage_types.rs` updated if you added new storage keys or structs
- [ ] `lib.rs` updated if you added a new module
- [ ] PR description explains what the change does and why

---

## Branching

- `main` is the stable branch
- Branch from `main` for your work: `git checkout -b feat/your-feature`
- Open your PR against `main`

---

## Reporting issues

Open an issue on GitHub describing:
- What you expected to happen
- What actually happened
- Steps to reproduce
- Rust and Stellar CLI versions (`rustc --version`, `stellar --version`)

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
