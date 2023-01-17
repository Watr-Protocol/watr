#!/usr/bin/env bash

steps=50
repeat=20
benchmarkOutput=./runtime/devnet/src/weights
benchmarkRuntimeName="devnet-dev"

pallets=(
    frame_system
    pallet_assets
    pallet_balances
    pallet_multisig
    pallet_session
    pallet_utility
    pallet_timestamp
    pallet_collator_selection
    cumulus_pallet_xcmp_queue
    pallet_collective
    pallet_identity
    pallet_scheduler
    pallet_treasury
    pallet_membership
)

for pallet in ${pallets[@]}
do
	output_file="${pallet//::/_}"

    target/release/water-node benchmark pallet \
		--chain=$benchmarkRuntimeName \
		--execution=wasm \
		--wasm-execution=compiled \
		--pallet=$pallet  \
		--extrinsic='*' \
		--steps=$steps  \
		--repeat=$repeat \
		--json \
		--header=./file_header.txt \
		--output="${benchmarkOutput}/${output_file}.rs"
done
