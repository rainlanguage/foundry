//! Return types for `eth_getProof`

use crate::eth::trie::KECCAK_NULL_RLP;
use alloy_primitives::{B256, U256};
use foundry_common::types::ToEthers;
use revm::primitives::KECCAK_EMPTY;

// reexport for convenience
pub use alloy_rpc_types::{
    EIP1186AccountProofResponse as AccountProof, EIP1186StorageProof as StorageProof,
};

/// Basic account type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicAccount {
    /// Nonce of the account.
    pub nonce: U256,
    /// Balance of the account.
    pub balance: U256,
    /// Storage root of the account.
    pub storage_root: B256,
    /// Code hash of the account.
    pub code_hash: B256,
}

impl Default for BasicAccount {
    fn default() -> Self {
        BasicAccount {
            balance: 0.into(),
            nonce: 0.into(),
            code_hash: KECCAK_EMPTY.to_ethers(),
            storage_root: KECCAK_NULL_RLP,
        }
    }
}
