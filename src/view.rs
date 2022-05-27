use crate::*;

#[near_bindgen]
impl Contract {
    // lấy ra merkle root của theo id của campaign
    pub fn airdrop_merkle_root(&self, airdrop_id: AirdropId) -> Option<String> {
        let result = self.merkle_roots_by_id.get(&airdrop_id);
        result
    }

    // danh sách các airdrop campaigns của 1 account
    pub fn airdrop_campaigns_by_account(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<AirdropId> {
        let campaigns = self.campaigns_by_account.get(&account_id);
        let campaigns = if let Some(campaigns) = campaigns {
            campaigns
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        campaigns
            .as_vector()
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .collect()
    }

    // number of campaigns by account
    pub fn number_airdrop_campaigns_by_account(&self, account_id: AccountId) -> U128 {
        let campaigns = self.campaigns_by_account.get(&account_id);
        let campaigns = if let Some(campaigns) = campaigns {
            return U128(campaigns.len() as u128);
        } else {
            return U128(0);
        };
    }

    // total number of campaigns
    pub fn total_number_airdrop_campaigns(&self) -> U128 {
        U128(self.spent_list_by_campaign.len() as u128)
    }

    pub fn get_owner_id(&self) -> String {
        self.owner_id.to_string()
    }

    pub fn get_ft_contract_by_campaign(&self, airdrop_id: AirdropId) -> AccountId {
        AccountId::try_from(self.ft_contract_by_campaign.get(&airdrop_id).unwrap()).unwrap()
    }
}
