#!/bin/bash
rm -rvf out

set -e
sh build.sh

rm -f .env
rm -rvf neardev

near dev-deploy out/rust_contract_events.wasm
source neardev/dev-account.env

CONTRACT=$CONTRACT_NAME
OWNER_ID=owner.$CONTRACT

echo "CONTRACT=$CONTRACT" > .env
echo "OWNER_ID=$OWNER_ID" >> .env

set -e
near create-account $OWNER_ID --masterAccount $CONTRACT --initialBalance 20
