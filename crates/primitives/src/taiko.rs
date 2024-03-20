#![allow(missing_docs)]

use revm_primitives::{Address, Bytes, B256};
use serde::{Deserialize, Serialize};

/// BlockMetadata represents a `BlockMetadata` struct defined in protocol.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaikoBlockMetadata {
    pub beneficiary: Address,
    pub gas_limit: u64,
    pub timestamp: u64,
    pub mix_hash: B256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_list: Option<Vec<Bytes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_block_id: Option<u64>,
    pub extra_data: Vec<u8>,
}

/// L1Origin represents a L1Origin of a L2 block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1Origin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<u64>,
    pub l2_block_hash: B256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l1_block_height: Option<u64>,
    pub l1_block_hash: B256,
}