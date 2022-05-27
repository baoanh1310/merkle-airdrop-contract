use crate::*;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

const GAS_FOR_FT_TRANSFER: Gas = Gas(15_000_000_000_000);
const XCC_GAS: Gas = Gas(20_000_000_000_000);

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_decimal(&self) -> u128;
}

pub trait ExtSelf {
    fn callback_decimal(&self) -> u128;
}

#[ext_contract(ext_ft)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_ft_metadata)]
pub trait FungibleTokenMeta {
	fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl Contract {
    pub fn claim_token(&self, airdrop_id: AirdropId, account_id: AccountId, amount: U128) {
    	ext_ft_metadata::ft_metadata(
            self.get_ft_contract_by_campaign(airdrop_id),
            1,
            GAS_FOR_FT_TRANSFER,
        )
        .then(ext_self::callback_decimal(
            env::predecessor_account_id(),
            0,
            XCC_GAS,
        ))
        // ext_ft::ft_transfer(
        //     account_id.clone(),
        //     amount,
        //     None,
        //     self.get_ft_contract_by_campaign(airdrop_id),
        //     1,
        //     GAS_FOR_FT_TRANSFER,
        // );
    }
}

#[near_bindgen]
impl ExtSelf for Contract {
	#[private]
    fn callback_decimal(&self) -> u128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(val) = near_sdk::serde_json::from_slice::<FungibleTokenMetadata>(&val) {
                    val.decimals.into()
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}