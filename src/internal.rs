use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_add_campaign_to_account(&mut self, account_id: &AccountId) -> AirdropId {
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
        let empty_lookup_map = UnorderedMap::new(prefix);
        self.spent_list_by_campaign
            .insert(&airdrop_id, &empty_lookup_map);
        airdrop_id
    }

    pub(crate) fn internal_check_issued_account(
        &self,
        airdrop_id: &AirdropId,
        account_id: &AccountId,
    ) -> bool {
        let account_map = self.spent_list_by_campaign.get(airdrop_id);
        let account_map = if let Some(account_map) = account_map {
            account_map
        } else {
            let mut prefix = Vec::with_capacity(33);
            prefix.push(b's');
            prefix.extend(env::sha256(account_id.as_bytes()));
            UnorderedMap::new(prefix)
        };
        if account_map.len() == 0 {
            return false;
        }
        let is_issued = account_map.get(account_id).unwrap();
        is_issued
    }

    pub(crate) fn internal_check_merkle_proof(
        &self,
        airdrop_id: &AirdropId,
        proof: &Vec<Proof>,
        amount: Balance,
    ) -> bool {
        // get merkle_root
        let merkle_root = self.merkle_roots_by_id.get(&airdrop_id).unwrap();

        let decode_root = hex::decode(merkle_root.as_str()).unwrap();
        let decode_root = String::from_utf8_lossy(&decode_root).to_string();

        let user_id = env::predecessor_account_id();
        let user_info = format!("{} {}", user_id.clone(), amount.to_string());
        let user_hash_buf = env::sha256(user_info.clone().as_bytes());
        let mut user_hash = String::from_utf8_lossy(&user_hash_buf).to_string();

        let mut buf = user_hash_buf.clone();

        for el in proof.iter() {
            let e = hex::decode(el.data.as_str()).unwrap();
            let position = el.position.as_str();

            let mut info = Vec::new();
            match position {
                "right" => {
                    info.extend_from_slice(&buf);
                    info.extend_from_slice(&e);
                }
                _ => {
                    info.extend_from_slice(&e);
                    info.extend_from_slice(&buf);
                }
            }
            buf = env::sha256(&info.clone());
        }

        user_hash = get_string_from_hash(buf);
        user_hash == decode_root.clone()
    }

    pub(crate) fn internal_add_account_to_claimed_list(&mut self, airdrop_id: &AirdropId) {
        let mut account_map = self.spent_list_by_campaign.get(airdrop_id).unwrap();
        let user_id = env::predecessor_account_id();
        account_map.insert(&user_id, &true);
        self.spent_list_by_campaign.insert(airdrop_id, &account_map);
    }
}
