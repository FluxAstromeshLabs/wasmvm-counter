# Flux Counter Program
This guide describes how to build, deploy, and interact with the `flux-counter` EVM program via our wasmvm-counter example.

## Prerequisites

* Install Metamask/Kelpr wallet.
* Go to www.astromesh.xyz > Built-in > Faucet > and claim `lux` token for gas fee.
* Install deps

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

## Build and Export Contract Artifacts
1. Clone the example repo https://github.com/FluxNFTLabs/wasmvm-counter

```
git clone https://github.com/FluxNFTLabs/wasmvm-counter
cd wasmvm-counter
```

2. Build project and generate schema

```
make build-release
cargo run
```

3. Verify these files exist

```
target/wasm32-unknown-unknown/release/wasmvm_counter.wasm
schema/wasmvm-counter.json
```

## Deploy Contract on Astromesh
1. On Astromesh, navigate to "Planes" > "WASMVM" > "Deploy".
2. Click "Upload wasm binary" and choose `wasmvm_counter.wasm`, then click "Store Contract".
3. Select your contract Code ID and paste in this instantiate msg, fill in your wallet address, sign and send transaction. When tx is processed, copy your contract address from popup window.

```
{
  "label": "wasmvm-counter",
  "admin": "<your-wallet-address>",
  "msg": {},
  "funds": []
}
```

## Interact with Contract on Astromesh

1. Go to "Planes" > "WASMVM" > "Interact".
2. Enter the contract address you just deployed.
3. Paste in `{"count": {}}` and click "Execute"
4. Paste in `{"count": {}}` again and click "Query" to retrieve counter value.
