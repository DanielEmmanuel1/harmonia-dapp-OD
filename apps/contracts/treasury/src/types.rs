extern crate alloc;

use soroban_sdk::{contracttype, Address, Symbol, symbol_short, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionLog {
    pub tx_id: Symbol,
    pub asset: Address,
    pub amount: i128,
    pub direction: Symbol, // "in" or "out"
    pub timestamp: u64,
    pub status: Symbol, // "pending" or "released"
    pub triggered_by: Address,
}

// This implementation is used by unwrap_or_default() in the contract
// It's a placeholder for when no transaction is found
impl Default for TransactionLog {
    fn default() -> Self {
        Self {
            tx_id: symbol_short!("default"),
            // Using simple zero address as placeholder
            asset: Address::from_str(&Env::default(), "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"),
            amount: 0,
            direction: symbol_short!("none"),
            timestamp: 0,
            status: symbol_short!("none"),
            // Using same zero address for triggered_by
            triggered_by: Address::from_str(&Env::default(), "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"),
        }
    }
}
