#![no_std]

// Veritix Pay — Smart contract logic coming soon.
// Contributors: see CONTRIBUTING.md for how to get started.

pub mod storage_types;
pub mod admin;
pub mod metadata;
pub mod allowance;
pub mod balance;

mod contract;

#[cfg(test)]
mod test;

pub use crate::contract::VeritixToken;
