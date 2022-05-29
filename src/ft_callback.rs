use crate::*;

const GAS_FOR_FT_TRANSFER: Gas = Gas(15_000_000_000_000);
const XCC_GAS: Gas = Gas(2_000_000_000_000);

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
    fn ft_transfer_call(
        &mut self,
        receiver_id: String,
        amount: String,
        memo: Option<String>,
        msg: String,
    ) -> U128;
}

#[ext_contract(ext_ft_metadata)]
pub trait FungibleTokenMetadataProvider {
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl Contract {
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
            ).into()
    }

    pub fn claim_token(&self, airdrop_id: AirdropId, amount: U128) -> Promise {
        let receiver_id = env::predecessor_account_id();
        ext_ft::ext(self.get_ft_contract_by_campaign(airdrop_id))
            .with_attached_deposit(1)
            .with_static_gas(XCC_GAS)
            .ft_transfer(
                receiver_id.clone(),
                amount,
                None
            )
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
}
