# Setup
Deploy the contract and initialize it:
```bash
sh deploy-testnet.sh
source .env

near call $CONTRACT new_default_meta '{"owner_id": "'$OWNER_ID'"}' --accountId $CONTRACT
```

# Examples of events

The smart contract emits an event anytime NFTs are minted, transferred, or burnt. 

## Mint
```bash
near call $CONTRACT nft_mint '{"token_id": "TOKEN", "receiver_id": "'$CONTRACT'", "token_metadata": {}}' --accountId $CONTRACT --deposit 0.01
```
The output should contain the event log:
```
	Log [dev-1649670901336-81485369365459]: EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"dev-1649670901336-81485369365459","token_ids":["TOKEN"]}]}
```

## Transfer
```bash
near call $CONTRACT nft_transfer '{"receiver_id": "'$OWNER_ID'", "token_id": "TOKEN"}' --accountId $CONTRACT --depositYocto 1
```
The output should contain the event log:
```
	Log [dev-1649670901336-81485369365459]: EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"old_owner_id":"owner.dev-1649670901336-81485369365459","new_owner_id":"owner.dev-1649670901336-81485369365459","token_ids":["TOKEN"]}]}
```

## Burn
```bash
near call $CONTRACT nft_burn '{"token_id": "TOKEN"}' --accountId $OWNER_ID --deposit 0.01
```
The output should contain the event log:
```
	Log [dev-1649670901336-81485369365459]: EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_burn","data":[{"owner_id":"owner.dev-1649670901336-81485369365459","token_ids":["TOKEN"]}]}
```