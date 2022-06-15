use crate::*;

const GAS_FOR_FT_TRANSFER: Gas = Gas(250_000_000_000_000);
const XCC_GAS: Gas = Gas(2_000_000_000_000);
const DEPOSIT_GAS: Gas = Gas(15_000_000_000_000);

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropArgs {
    pub merkle_root: String,
    pub ft_account_id: String,
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_decimal(&self) -> u128;
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
    fn storage_deposit(&mut self, account_id: AccountId);
    // near call contract storage_deposit '{}' --accountId alice --amount 0.1
}

#[ext_contract(ext_storage)]
pub trait StorageManagement {
    fn storage_deposit(&mut self, account_id: Option<String>, registration_only: Option<bool>) -> StorageBalance;
}

#[ext_contract(ext_ft_metadata)]
pub trait FungibleTokenMetadataProvider {
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl Contract {
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Parse msg
        let AirdropArgs {
            merkle_root,
            ft_account_id,
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid airdrop arguments");
        assert!(
            near_sdk::env::is_valid_account_id(ft_account_id.clone().as_bytes()),
            "Invalid fungible token contract id"
        );

        self.create_airdrop(merkle_root, ft_account_id);
        near_sdk::PromiseOrValue::Value(U128(0))
    }

    pub fn create_airdrop(&mut self, merkle_root: String, ft_account_id: String) {
        let campaign_owner_id = env::predecessor_account_id();
        let airdrop_id = self.internal_add_campaign_to_account(&campaign_owner_id);
        self.merkle_roots_by_id.insert(&airdrop_id, &merkle_root);
        self.internal_add_ft_contract_to_campaign(&airdrop_id, &ft_account_id);
    }

    pub fn get_ft_decimals(&self, airdrop_id: AirdropId) -> PromiseOrValue<U128> {
        ext_ft_metadata::ext(self.get_ft_contract_by_campaign(airdrop_id))
            .with_attached_deposit(1)
            .with_static_gas(XCC_GAS)
            .ft_metadata()
            .then(
                ext_self::ext(env::current_account_id())
                    .with_attached_deposit(0)
                    .with_static_gas(XCC_GAS)
                    .callback_decimal(),
            )
            .into()
    }

    #[private]
    pub fn callback_decimal(&self) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(val) = near_sdk::serde_json::from_slice::<FungibleTokenMetadata>(&val) {
                    let decimals = val.decimals as u128;
                    U128(decimals)
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    pub fn claim_token(&self, airdrop_id: AirdropId, amount: U128) -> Promise {
        let receiver_id = env::predecessor_account_id();

        ext_storage::ext(self.get_ft_contract_by_campaign(airdrop_id))
            .with_attached_deposit(1250000000000000000000)
            .with_static_gas(XCC_GAS)
            .storage_deposit(Some(receiver_id.to_string().clone()), Some(true))
            .then(
                ext_ft::ext(self.get_ft_contract_by_campaign(airdrop_id))
                    .with_attached_deposit(1)
                    .with_static_gas(XCC_GAS)
                    .ft_transfer(receiver_id.clone(), amount, None)
            )
    }
}
