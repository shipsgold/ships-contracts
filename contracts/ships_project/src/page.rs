use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct PaginationOptions {
    pub limit: u64,
    pub from: u64,
    pub reverse: bool,
}

impl Default for PaginationOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            from : 0,
            reverse: false
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct PaginationResponse {
    limit: u64,
    from: u64,
    total: u64
}