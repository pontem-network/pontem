# Substrate Move VM

Substrate node template with Move VM pallet on board.

**It's alpha version. Work in progress, so use it at your own risk.**

* [Documentation](https://docs.pontem.network).

## Installation

Read [official documentation](https://docs.pontem.network/02.-getting-started/local_node).

## Local Relaychain & Parachain Launch

Current version built with Nimbus consensus and Parachain Staking implementation. 
Requires relay chain to work correctly.

### Using polka-launch

Install [polkadot-launch](https://github.com/paritytech/polkadot-launch).

**Note:** you must have polkadot node `v0.9.8` compiled and built placed in `../polkadot/target/release/`.
To use different localion you can modify `./launch-config.json`.

**Build Pontem:**

```sh
cd sp-move
make build
```

Create keystore path for Pontem:

```sh
mkdir ~/.pontem/keystore-1 # Base path
```

**Add Nimbus key:**

```sh
# Use "//Alice" for URI.
./target/release/mv-node key insert --keystore-path ~/.pontem/keystore-1 --key-type nmbs
```

```sh
# run mv-node
polkadot-launch ./launch-config.json
```

Wait for an minute.

Observe `9946.log` to verify that the node was launched successfully and is producing blocks, also you can use [Web UI](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/explorer).

```sh
tail -f ./9946.log
```

### Manually

**Build Polkadot:**

```sh
git clone https://github.com/paritytech/polkadot.git
cd polkadot 
git checkout v0.9.8
cargo build --release
```

**Launch Polkadot Relay Chain:**

```sh
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json
./target/release/polkadot --chain rococo-local-cfde.json --alice --tmp
./target/release/polkadot --chain rococo-local-cfde.json --bob --tmp --port 30334 # In a separate terminal
``` 

**Build Pontem:**

```sh
cd sp-move
make build
```

Create keystore path for Pontem:

```sh
mkdir ~/.pontem/keystore-1 # Base path
```

**Add Nimbus key:**

```sh
# Use "//Alice" for URI.
./target/release/mv-node key insert --keystore-path ~/.pontem/keystore-1 --key-type nmbs
```

Launch parachain node as collator:

```sh
./target/release/mv-node export-genesis-state --parachain-id 2000 > genesis-state
./target/release/mv-node export-genesis-wasm > genesis-wasm
./target/release/mv-node --collator --tmp --keystore-path ~/.pontem/keystore-1 --parachain-id 2000 --port 40335 --ws-port 9946 -- --execution wasm --chain ../polkadot/rococo-local-cfde.json --port 30335
```

**Register the parachain**

1. Navigate to [sudo UI](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/sudo) in Relay Chain.
2. Choose `parasSudoWrapper`.
3. Choose `sudoScheduleParaInitialize(id, genesis)` method.
4. Change `id` to `2000`.
5. Upload `genesis-state` to `genesisHead` field.
6. Upload `gensis-wasm` to `validationCode` field.
7. Change `parachain` field to `Yes`.
8. Send transaction.
9. Restart `mv-node`.

## Metadata

Metadata for Polkadot JS:

```json
{
  "Balance": "u128",
  "RoundIndex": "u32",
  "AuthorId": "[u8;32]",
  "RegistrationInfo": {
    "account": "AccountId",
    "deposit": "Balance"
  }
}
```

## Documentation

See [Move VM Pallet documentation](https://docs.pontem.network/02.-getting-started/getting_started).

## LICENSE

See [LICENSE](/LICENSE).
