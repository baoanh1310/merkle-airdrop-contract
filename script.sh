#!/bin/bash
set -e

near delete merkle.icebear.testnet icebear.testnet
near create-account merkle.icebear.testnet --masterAccount icebear.testnet --initialBalance 10
./build.sh
near deploy merkle.icebear.testnet --wasmFile res/merkle_airdrop.wasm
near call merkle.icebear.testnet new '{"owner_id": "merkle.icebear.testnet"}' --accountId merkle.icebear.testnet

# near call merkle.icebear.testnet create_airdrop '{"merkle_root": "98fd4e7fbfe022cee50f418f005d772e20c8eeca73782c6eb10d74964d98b065", "ft_account_id": "ft-ceus.icebear.testnet", "ft_balance": 10000}' --accountId icebear.testnet
# near view merkle.icebear.testnet total_number_airdrop_campaigns
# near view merkle.icebear.testnet get_ft_contract_by_campaign '{"airdrop_id": 1}'
# near view merkle.icebear.testnet airdrop_campaigns_by_account '{"account_id": "icebear.testnet", "from_index": "0", "limit": 10}'
# near view merkle.icebear.testnet number_airdrop_campaigns_by_account '{"account_id": "icebear.testnet"}'
# near view merkle.icebear.testnet airdrop_merkle_root '{"airdrop_id": 1}'
# near call merkle.icebear.testnet claim '{"airdrop_id": 1, "proof": [{ "position": "right", "data": "1b6258050607374c1a83fe03a51772ad8dba4b43ce9e6a0bb91eea2fe5b39590" }, { "position": "right", "data": "6625f781853af79d731d131748e19c53e1959a1ffcef2870b8e8e74da3df0885" }], "amount": 10}' --accountId icebear.testnet
# near view merkle.icebear.testnet
# near call merkle.icebear.testnet get_ft_decimals '{"airdrop_id": 1, "account_id": "icebear.testnet", "amount": "10000"}' --accountId icebear.testnet --gas 3000000000000000
