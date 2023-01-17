#!/usr/bin/env bash

steps=50
repeat=20
runtimeName=$1
benchmarkOutput=./runtime/$runtimeName/src/weights
benchmarkRuntimeName="$runtimeName-dev"

if [[ $runtimeName == "devnet" ]] || [[ $runtimeName == "mainnet" ]]; then
    pallets=(
        frame_system
        pallet_assets
        pallet_balances
        pallet_multisig
        pallet_preimage
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
else
	echo "'$runtimeName' pallet list not found in benchmarks-ci.sh"
	exit 1
fi

for pallet in ${pallets[@]}
do
	output_file="${pallet//::/_}"

    target/release/watr-node benchmark pallet \
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
