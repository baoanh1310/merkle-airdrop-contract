use crate::internal::*;
use crate::utils::*;
use crate::view::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{env, near_bindgen, AccountId, Balance};

mod internal;
mod utils;
mod view;

pub type AirdropId = u128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub merkle_roots_by_id: LookupMap<AirdropId, String>,
    pub campaigns_by_account: LookupMap<AccountId, UnorderedSet<AirdropId>>,
    pub spent_list_by_campaign: UnorderedMap<AirdropId, LookupMap<AccountId, bool>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            merkle_roots_by_id: LookupMap::new(b"c"),
            campaigns_by_account: LookupMap::new(b"u"),
            spent_list_by_campaign: UnorderedMap::new(b"e"),
        }
    }

    #[payable]
    pub fn create_airdrop(&mut self, merkle_root: String, ft_account_id: AccountId, ft_balance: Balance) {
        let campaign_owner_id = env::predecessor_account_id();
        self.internal_add_campaign_to_account(&campaign_owner_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
}
