use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_add_campaign_to_account(&mut self, account_id: &AccountId) {
        // insert new airdrop_id to spent_list_by_campaign
        let mut campaigns_set = self
            .campaigns_by_account
            .get(account_id)
            .unwrap_or_else(|| UnorderedSet::new(b'f'));

        let airdrop_id = self.spent_list_by_campaign.len() as u128 + 1 as u128;
        campaigns_set.insert(&airdrop_id);
        self.campaigns_by_account.insert(account_id, &campaigns_set);

        // init new mapping in spend_list_by_campaign
        // generate prefix for nested LookupMap collection
        let mut prefix = Vec::with_capacity(33);
        prefix.push(b's');
        prefix.extend(env::sha256(account_id.as_bytes()));
        let empty_lookup_map = LookupMap::new(prefix);
        self.spent_list_by_campaign
            .insert(&airdrop_id, &empty_lookup_map);
    }
}
