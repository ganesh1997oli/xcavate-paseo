#!/bin/bash

pallets=(
	pallet_sudo
    frame_system
    pallet_balances
    pallet_session
    pallet_timestamp
    pallet_message_queue
    pallet_collator_selection    
    cumulus_pallet_parachain_system
    cumulus_pallet_xcmp_queue
    #pallet_xcm
)

# Generate weights
for pallet_name in "${pallets[@]}"; do
    ./target/release/parachain-template-node benchmark pallet \
        --pallet $pallet_name \
        --extrinsic "*" \
        --steps 50 \
        --repeat 20 \
        --output ./runtime/src/weights/$pallet_name.rs
done