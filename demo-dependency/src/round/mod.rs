
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::schemars::JsonSchema;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq,JsonSchema
)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub enum RoundStatus {
    Running,
    Pending,
}
