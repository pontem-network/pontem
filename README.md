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

Build Pontem:

```sh
cd sp-move
make build
```

Create keystore path for Pontem:

```sh
mkdir -p ~/.pontem/keystore-1 # Base path
```

Add Nimbus key:

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

Build Polkadot:

```sh
git clone https://github.com/paritytech/polkadot.git
cd polkadot
git checkout v0.9.8
cargo build --release
```

Launch Polkadot Relay Chain:

```sh
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json
./target/release/polkadot --chain rococo-local-cfde.json --alice --tmp
./target/release/polkadot --chain rococo-local-cfde.json --bob --tmp --port 30334 # In a separate terminal
```

Build Pontem:

```sh
cd sp-move
make build
```

Create keystore path for Pontem:

```sh
mkdir -p ~/.pontem/keystore-1 # Base path
```

Add Nimbus key:

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

Register the parachain:

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
  "Balance": "u64",
  "RoundIndex": "u32",
  "AuthorId": "[u8;32]",
  "RegistrationInfo": {
    "account": "AccountId",
    "deposit": "Balance"
  }
}
```

* Current amount of top collator is 8.
* Block time is 12 seconds.
* There is 1 hour rounds.

## Connect as a new collator

Create keystore path for the new key:

```sh
mkdir ~/.pontem/keystore-2 # Base path
```

Add new Nimbus key:

```sh
# Use "//Bob" for URI.
./target/release/mv-node key insert --keystore-path ~/.pontem/keystore-2 --key-type nmbs
```

Get your public key:

```sh
# Use "//Bob" for dev purposes URI.
./target/release/mv-node key inspect
```

You will see something like:

```sh
Secret Key URI `//Bib` is account:
Secret seed:       0x02ca07977bdc4c93b5e00fcbb991b4e8ae20d05444153fd968e04bed6b4946e7
Public key (hex):  0xb832ced5ca2de9fe76ef101d8ab1b8dd778e1ab5a809d019c57b78e45ecbaa56
Public key (SS58): 5GEDm6TY5apP4bhwuTtTzA7z9vHbCL1V2D5nE8sPga6WKhNH
Account ID:        0xb832ced5ca2de9fe76ef101d8ab1b8dd778e1ab5a809d019c57b78e45ecbaa56
SS58 Address:      5GEDm6TY5apP4bhwuTtTzA7z9vHbCL1V2D5nE8sPga6WKhNH
```

Copy `Public key (hex)` as your public key, it's going to be your validator public key.
Now you need to map your public key with your account.

Send new transaction to map your public key with your account:

1. Navigate to [extrinsics](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/extrinsics).
2. Choose `authorMapping` pallet.
3. Choose `addAssociation(author_id)` function.
4. Put your public key in `author_id` field.
5. Send transaction from your account.

Now create your validator:

1. Navigate to [extrinsics](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/extrinsics).
2. Choose `parachainStaking` pallet.
3. Choose `joinCandidates(bond, candidate_count)` function.
4. Put amount to bond in PONT tokens.
5. For candidate_count use `1`.
6. Send transaction.

Now time to launch your node.

**If you used polkadot-launch to launch everything:**

```sh
/target/release/mv-node --collator \
     --tmp \
    --keystore-path ~/.pontem/keystore-2  \
    --parachain-id 2000 \
    --port 40338 \
    --ws-port 9947 \
    --bootnodes <bootnode> \
    -- --execution wasm --chain ./rococo-local.json --port 40336
```

Replace bootnode with peer address from result of following command:

```ssh
cat 9946.log | grep 40335 # Something like: /ip4/127.0.0.1/tcp/40335/p2p/12D3KooWM1a6mBNyvZwbN5T3sDuYuxgxmNoj83CnFqaHJzaB8GYV
```

**If you used manual method:**

```sh
/target/release/mv-node --collator \
    --tmp \
    --keystore-path ~/.pontem/keystore-2  \
    --parachain-id 2000 \
    --port 40338 \
    --ws-port 9947 \
    --bootnodes <bootnode> \
    -- --execution wasm --chain ../polkadot/rococo-local-cfde.json --port 40336
```

Good documentation also can be found in [Moonriver/Moonbeam Docs](https://docs.moonbeam.network/staking/stake/).

## Documentation

See [Move VM Pallet documentation](https://docs.pontem.network/02.-getting-started/getting_started).

## LICENSE

See [LICENSE](/LICENSE).
