# Veritix Contracts

Veritix Contracts contains the **on-chain logic** for the Veritix ticketing platform, built using **Rust** and **Soroban**, Stellar’s smart contract platform.

These contracts define the core rules that govern ticket issuance, ownership, transfers, validation, and settlement, ensuring that critical ticketing operations are **transparent, tamper-resistant, and verifiable on the Stellar network**.

---

## Overview

Veritix is a blockchain-powered ticketing system designed to prevent fraud, double spending, and unauthorized resale while giving event organizers full control over ticket rules.

This repository focuses exclusively on **on-chain concerns**, while the backend and frontend handle orchestration, UI, and off-chain data.

The contracts are responsible for:
- Enforcing ticket ownership rules
- Validating transfers and resale conditions
- Anchoring ticket lifecycle events on Stellar
- Providing a verifiable source of truth for ticket state

---

## Why Stellar & Soroban

Stellar is a decentralized network optimized for real-world applications that require speed, low cost, and security.  
Soroban is Stellar’s smart contract platform, designed with safety and performance in mind.

Key reasons Veritix uses Stellar and Soroban:
- Deterministic execution and predictable fees
- Fast finality suitable for real-time ticket validation
- Strong Rust-based safety guarantees
- Native integration with the Stellar ecosystem
- Designed for production-grade financial and utility applications

---

## Core On-Chain Features

### Ticket Issuance
- Register new tickets on-chain
- Bind tickets to event identifiers
- Associate tickets with an owner address

### Ownership & Transfers
- Enforce ownership checks for all actions
- Controlled ticket transfers
- Optional transfer limits or lock periods
- Organizer-defined resale rules

### Ticket Validation
- Verify ticket authenticity
- Prevent double usage
- Mark tickets as used or invalid after entry

### Event Anchoring
- Anchor critical event actions on Stellar
- Maintain an immutable audit trail
- Enable independent verification by third parties

### Organizer Controls
- Define ticket policies at creation
- Enable or disable transfers
- Enforce event-specific rules

---

## Repository Structure


veritix-contract/
├── contracts/
│   ├── ticket/
│   │   ├── lib.rs
│   │   └── types.rs
│   ├── event/
│   │   ├── lib.rs
│   │   └── types.rs
├── tests/
│   └── contract_tests.rs
├── Cargo.toml
├── Cargo.lock
└── README.md

---

## Development Setup

### Requirements
- Rust (stable)
- Soroban CLI
- Stellar testnet access

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
