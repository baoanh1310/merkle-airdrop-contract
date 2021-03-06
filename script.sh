#!/bin/bash
set -e

near delete merkle1.icebear.testnet icebear.testnet
near create-account merkle1.icebear.testnet --masterAccount icebear.testnet --initialBalance 10
./build.sh
near deploy merkle1.icebear.testnet --wasmFile res/merkle_airdrop.wasm
near call merkle1.icebear.testnet new '{"owner_id": "merkle1.icebear.testnet"}' --accountId merkle1.icebear.testnet

# near call merkle1.icebear.testnet create_airdrop '{"merkle_root": "98fd4e7fbfe022cee50f418f005d772e20c8eeca73782c6eb10d74964d98b065", "ft_account_id": "ft-ceus.icebear.testnet", "ft_balance": 10000}' --accountId icebear.testnet
near view merkle1.icebear.testnet total_number_airdrop_campaigns
# near view merkle1.icebear.testnet get_ft_contract_by_campaign '{"airdrop_id": 1}'
# near view merkle1.icebear.testnet airdrop_campaigns_by_account '{"account_id": "icebear.testnet", "from_index": "0", "limit": 10}'
# near view merkle1.icebear.testnet number_airdrop_campaigns_by_account '{"account_id": "icebear.testnet"}'
# near view merkle1.icebear.testnet airdrop_merkle_root '{"airdrop_id": 3}'
# near call ft-ceus.icebear.testnet storage_deposit '{"account_id": "merkle1.icebear.testnet"}' --accountId merkle1.icebear.testnet --deposit 0.125
# near call ft-ceus.icebear.testnet ft_transfer '{"receiver_id": "merkle1.icebear.testnet", "amount": "1000000"}' --accountId ft-ceus.icebear.testnet --amount 0.000000000000000000000001
# near call merkle1.icebear.testnet claim '{"airdrop_id": 1, "proof": [{ "position": "right", "data": "1b6258050607374c1a83fe03a51772ad8dba4b43ce9e6a0bb91eea2fe5b39590" }, { "position": "right", "data": "6625f781853af79d731d131748e19c53e1959a1ffcef2870b8e8e74da3df0885" }], "amount": 10}' --accountId icebear.testnet
# near call merkle1.icebear.testnet claim '{"airdrop_id": 1, "proof": [{ "position": "left", "data": "413a4943f4b810d97809d5408ca04dfe8b96fe48398167bdd3fc85c49b0fe3cb" }, { "position": "right", "data": "6625f781853af79d731d131748e19c53e1959a1ffcef2870b8e8e74da3df0885" }], "amount": 20}' --accountId ceus.testnet
# near view merkle1.icebear.testnet check_issued_account '{"airdrop_id": 1, "account_id": "icebear.testnet"}'
# near call merkle1.icebear.testnet get_ft_decimals '{"airdrop_id": 1}' --accountId icebear.testnet

# near view ft-ceus.icebear.testnet ft_balance_of '{"account_id": "icebear.testnet"}'
# near view ft-ceus.icebear.testnet ft_balance_of '{"account_id": "johanliebert.testnet"}'
# near view ceus-token.icebear.testnet ft_balance_of '{"account_id": "johanliebert.testnet"}'

# near view ft-ceus.icebear.testnet ft_balance_of '{"account_id": "merkle1.icebear.testnet"}'
# near view merkle1.icebear.testnet get_all_campaigns

# near call merkle1.icebear.testnet create_airdrop '{"merkle_root": "e8ee14461ae89a8c0f9e291cf6500bf502a6758962d370954456ab4e39cf30ae", "ft_account_id": "ft-ceus.icebear.testnet", "ft_balance": 40}' --accountId icebear.testnet
# near call merkle1.icebear.testnet claim '{"airdrop_id": 2, "proof": [{ "position": "left", "data": "e4bb5666a258de82f56402a1a1dfa0c18d1566dd675bf0cd230210a46b4d86de" }], "amount": 20}' --accountId johanliebert.testnet
# near call merkle1.icebear.testnet claim '{"airdrop_id": 3, "proof": [{ "position": "left", "data": "e4bb5666a258de82f56402a1a1dfa0c18d1566dd675bf0cd230210a46b4d86de" }], "amount": 20}' --accountId johanliebert.testnet
# near call merkle1.icebear.testnet claim '{"airdrop_id": 2, "proof": [{ "position": "right", "data": "7b71155867b140bcd8f3d747a744c5a18e4a5f182474d4e5fbc17f165211d48f" }], "amount": 20}' --accountId icebear.testnet
# near call merkle1.icebear.testnet claim '{"airdrop_id": 3, "proof": [{ "position": "left", "data": "e4bb5666a258de82f56402a1a1dfa0c18d1566dd675bf0cd230210a46b4d86de" }], "amount": 20}' --accountId johanliebert.testnet

# near call ft-ceus.icebear.testnet storage_deposit '{"account_id": "johanliebert.testnet"}' --accountId icebear.testnet --deposit 0.125
# near call ceus-token.icebear.testnet storage_deposit '{"account_id": "johanliebert.testnet"}' --accountId johanliebert.testnet --deposit 0.00125
