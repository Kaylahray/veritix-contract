# Veritix Pay

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Built With Rust](https://img.shields.io/badge/Built%20With-Rust-orange.svg)
![Network: Stellar / Soroban](https://img.shields.io/badge/Network-Stellar%20%2F%20Soroban-7b5ea7.svg)
![Status: In Development](https://img.shields.io/badge/Status-In%20Development-yellow.svg)

On-chain payment infrastructure for the Veritix ticketing platform, built with Rust and Soroban on the Stellar network.

---

## Overview

Veritix Pay is the payment layer of a blockchain-based ticketing system. It lives entirely on-chain as a Soroban smart contract and handles all financial operations that power the Veritix platform — from a fan buying a ticket to an organizer receiving settlement funds after an event.

The contract handles **token transfers**, **escrow for ticket purchases**, **recurring payments**, **payment splitting between organizers, artists, and venues**, and **dispute resolution** when something goes wrong. Each of these is designed as a focused, composable module that shares a common storage layout and authorization model.

This project is currently being built in the open. The contract source files have been cleared and contributors can claim open GitHub Issues to build individual modules from scratch. If you want to contribute to a real Soroban project, this is a great place to start — see the [Contributing](#contributing) section below.

---

## Why Stellar & Soroban

- **Deterministic execution and predictable fees** — no gas spikes or unpredictable costs
- **Fast finality** — Stellar's 5-second finality is suitable for real-time ticket validation and payment confirmation
- **Rust-based safety guarantees** — Soroban contracts are written in Rust, giving strong compile-time correctness checks
- **Native Stellar ecosystem integration** — works directly with Stellar assets and accounts, no bridges required

---

## Contract Modules (Planned)

These modules will be built by contributors picking up open issues. The entry point is `src/lib.rs`.

| Module | File | Status | Description |
|--------|------|--------|-------------|
| Token Core | `contract.rs` | 🔜 Open | Mint, burn, transfer, approve |
| Escrow | `escrow.rs` | 🔜 Open | Create, release, and refund escrow holds |
| Recurring Payments | `recurring.rs` | 🔜 Open | Set up and execute recurring charges |
| Payment Splitter | `splitter.rs` | 🔜 Open | Split a payment between multiple parties |
| Dispute Resolution | `dispute.rs` | 🔜 Open | Open and resolve payment disputes |
| Admin | `admin.rs` | 🔜 Open | Admin address controls |
| Storage Types | `storage_types.rs` | 🔜 Open | Shared `DataKey` enum and struct definitions |
| Tests | `test.rs` | 🔜 Open | Unit test suite |

---

## Getting Started

### Prerequisites

| Tool | Notes | Install |
|------|-------|---------|
| Rust (stable) | Required to compile Soroban contracts | https://rustup.rs |
| wasm32 target | Required build target | `rustup target add wasm32-unknown-unknown` |
| Stellar CLI (latest) | For building and deploying | `cargo install stellar-cli` |

Verify your setup:

```bash
rustc --version
stellar --version
```

### Clone and Build

```bash
git clone https://github.com/Lead-Studios/veritix-contract.git
cd veritix-contract/veritixpay/contract/token

make build    # compile to WASM
make test     # run tests
make fmt      # format code
make clean    # remove build artifacts
```

---

## Project Structure

The repository is in a clean-slate state. Only `lib.rs` exists in `src/` — contributors will build out the modules by picking up issues.

```
veritixpay/
├── contract/
│   └── token/
│       ├── src/
│       │   └── lib.rs          # Entry point — start here
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

---

## Contributing

Contributions are welcome and actively encouraged. This project is structured so that each contract module is an independent issue that any contributor can pick up.

To get started:
1. Browse [open issues](https://github.com/Lead-Studios/veritix-contract/issues)
2. Comment on one to get assigned
3. Branch from `main`, build your module, write tests, and open a PR

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide — including project structure, how to add a module, storage conventions, authorization rules, and the PR checklist.

---

## Open Source Wave

This project is part of an active open-source funding wave on [Drips Network](https://www.drips.network/). Contributors who build meaningful features may be eligible for rewards. Build something real, on a real chain, with real incentives.

---

## Related Repositories

- **Backend:** https://github.com/Lead-Studios/veritix-backend
- **Web client:** https://github.com/Lead-Studios/veritix-web

---

## License

MIT
