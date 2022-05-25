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
    pub fn create_airdrop(
        &mut self,
        merkle_root: String,
        ft_account_id: AccountId,
        ft_balance: Balance,
    ) {
        let campaign_owner_id = env::predecessor_account_id();
        let airdrop_id = self.internal_add_campaign_to_account(&campaign_owner_id);
        self.merkle_roots_by_id.insert(&airdrop_id, &merkle_root);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract = Contract::new(accounts(1));

        assert_eq!(contract.owner_id, accounts(1));
        assert_eq!(contract.total_number_airdrop_campaigns(), U128(0));
    }

    #[test]
    fn test_create_airdrop() {
        let context = get_context(false);
        testing_env!(context.build());

        let mut contract = Contract::new(accounts(1));

        contract.create_airdrop(String::from("0x1234567"), accounts(2), 10000 as u128);
        assert_eq!(contract.total_number_airdrop_campaigns(), U128(1));
        assert_eq!(contract.airdrop_merkle_root(1 as u128).unwrap(), String::from("0x1234567"), "Merkle root failed");
        assert_eq!(contract.number_airdrop_campaigns_by_account(accounts(0)), U128(1), "Num campaign by account failed");
    }
}
