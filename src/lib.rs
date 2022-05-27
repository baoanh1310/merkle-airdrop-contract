use crate::ft_callback::*;
use crate::internal::*;
use crate::utils::*;
use crate::view::*;
use hex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Balance, Gas, Promise, PromiseResult, PanicOnDefault};
use std::convert::{TryFrom, TryInto};

mod ft_callback;
mod internal;
mod utils;
mod view;

pub type AirdropId = u128;

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Proof {
    pub position: String,
    pub data: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub merkle_roots_by_id: LookupMap<AirdropId, String>,
    pub campaigns_by_account: LookupMap<AccountId, UnorderedSet<AirdropId>>,
    pub spent_list_by_campaign: UnorderedMap<AirdropId, UnorderedMap<AccountId, bool>>,
    pub ft_contract_by_campaign: LookupMap<AirdropId, String>,
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
            ft_contract_by_campaign: LookupMap::new(b"h"),
        }
    }

    #[payable]
    pub fn create_airdrop(
        &mut self,
        merkle_root: String,
        ft_account_id: String,
        ft_balance: Balance,
    ) {
        let campaign_owner_id = env::predecessor_account_id();
        let airdrop_id = self.internal_add_campaign_to_account(&campaign_owner_id);
        self.merkle_roots_by_id.insert(&airdrop_id, &merkle_root);
        self.internal_add_ft_contract_to_campaign(&airdrop_id, &ft_account_id);
    }

    #[payable]
    pub fn claim(&mut self, airdrop_id: AirdropId, proof: Vec<Proof>, amount: Balance) {
        let user_id = env::predecessor_account_id();
        let is_issued = self.internal_check_issued_account(&airdrop_id, &user_id);
        assert_eq!(is_issued, false, "{} issued before!", user_id.clone());

        // println!("Issue account: {}", user_id.clone());

        assert_eq!(
            self.internal_check_merkle_proof(&airdrop_id, &proof, amount),
            true,
            "Your proof is invalid"
        );

        self.internal_add_account_to_claimed_list(&airdrop_id);

        self.claim_token(airdrop_id, user_id, U128(amount));
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
            .current_account_id(accounts(1))
            .signer_account_id(accounts(1))
            .predecessor_account_id(accounts(1))
            .is_view(is_view);

        builder
    }

    fn get_sample_proof() -> Vec<Proof> {
        let mut proof = Vec::new();
        proof.push(Proof {
            position: String::from("right"),
            data: String::from("e5974a9b4c89647842806f89429de6a0de3339bfecb56158508f86f8fd75bb11"),
        });
        proof.push(Proof {
            position: String::from("right"),
            data: String::from("73832786079a89e6f348539792c5d47112735731d3e80e51371c345e124cd0ff"),
        });

        proof
    }

    fn get_other_sample_proof() -> Vec<Proof> {
        let mut proof = Vec::new();
        proof.push(Proof {
            position: String::from("left"),
            data: String::from("0a63a38fbe51238ec101c9d469e1b7d7eff4934e82bbd06de32dde19a9d55ca0"),
        });
        proof.push(Proof {
            position: String::from("right"),
            data: String::from("73832786079a89e6f348539792c5d47112735731d3e80e51371c345e124cd0ff"),
        });

        proof
    }

    const SAMPLE_ROOT: &str = "6db3f929f60b1fe93d85e1665b4a7b8bfdb21d118e99d32bb7038113325a4eca";

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

        contract.create_airdrop(
            String::from(SAMPLE_ROOT),
            accounts(5).to_string(),
            10000 as u128,
        );
        assert_eq!(contract.total_number_airdrop_campaigns(), U128(1));
        assert_eq!(
            contract.airdrop_merkle_root(1 as u128).unwrap(),
            String::from(SAMPLE_ROOT),
            "Merkle root failed"
        );
        assert_eq!(
            contract.number_airdrop_campaigns_by_account(accounts(1)),
            U128(1),
            "Num campaign by account failed"
        );
        assert_eq!(contract.get_ft_contract_by_campaign(1 as u128), accounts(5));
    }

    #[test]
    fn test_claim() {
        let context = get_context(false);
        testing_env!(context.build());

        let mut contract = Contract::new(accounts(3));

        contract.create_airdrop(
            String::from(SAMPLE_ROOT),
            accounts(2).to_string(),
            10000 as u128,
        );
        // println!("Spent: {:?}", contract.)

        assert_eq!(contract.total_number_airdrop_campaigns(), U128(1));
        contract.claim(1 as u128, get_other_sample_proof(), 20);
    }
}
