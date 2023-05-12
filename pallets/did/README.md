# DID Pallet

`pallet-did` provides DID W3C capabilities

## Generate weights
```
cargo build --release --features runtime-benchmarks

./target/release/watr-node benchmark pallet --chain=devnet-dev --execution=wasm --wasm-execution=compiled --pallet=pallet_did --extrinsic=* --steps=50  --repeat=20 --heap-pages=4096 --output=./pallets/did/src/weights.rs --header=./file_header.txt --template=./scripts/frame-weight-template.hbs
```
