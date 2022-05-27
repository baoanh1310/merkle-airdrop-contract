use crate::*;

const GAS_FOR_FT_TRANSFER: Gas = Gas(15_000_000_000_000);
const XCC_GAS: Gas = Gas(20_000_000_000_000);

#[ext_contract(ext_self)]
trait ExtSelf {
    fn callback_decimal(&self) -> u128;
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_ft_metadata)]
pub trait FungibleTokenMetadataProvider {
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl Contract {
    pub fn get_ft_decimals(&self, airdrop_id: AirdropId) -> Promise {
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
    }

    #[private]
    pub fn callback_decimal(&self) -> u128 {
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
