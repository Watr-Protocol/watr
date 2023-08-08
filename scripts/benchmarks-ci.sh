#!/usr/bin/env bash

steps=50
repeat=20
runtimeName=$1
benchmarkOutput=./runtime/$runtimeName/src/weights
benchmarkRuntimeName="$runtimeName-dev"

pallets=($(
  target/release/watr-node benchmark pallet --list --chain=$benchmarkRuntimeName |\
    tail -n+2 |\
    cut -d',' -f1 |\
    sort |\
    uniq
))

for pallet in ${pallets[@]}
do
	output_file="${pallet//::/_}"

    target/release/watr-node benchmark pallet \
		--chain=$benchmarkRuntimeName \
		--wasm-execution=compiled \
		--pallet=$pallet  \
		--extrinsic='*' \
		--steps=$steps  \
		--repeat=$repeat \
		--json \
		--header=./file_header.txt \
		--output="${benchmarkOutput}/${output_file}.rs"
done
