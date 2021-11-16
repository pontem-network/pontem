# Pontem

Pontem parachain node with [Move VM pallet](/pallets/sp-mvm/) on board.

* [Pontem docs](https://docs.pontem.network) - if you want to learn about Pontem network.
* [Validator docs](https://docs.pontem.network/03.-staking/collator) - if you want to launch a validator node.
* [Bootstrap](https://github.com/pontem-network/bootstrap) - if you want to up node quickly (not for development).
 
## Local Relaychain & Parachain Launch

Current version built with Nimbus consensus and Parachain Staking implementation.
Requires relay chain to work correctly.

### Running in dev-mode

There is a possibility to run a single node in development mode, without any consensus involved.

Add `--dev-service` flag to `cargo run` command to run a single node with disabled consensus:

**IMPORTANT NOTE:** the node with enabled `--dev-service` flag generating blocks when needed (e.g. when a new transaction appears).

```sh
cargo run --release -- --dev --dev-service --tmp
```

Use `--sealing` argument to select sealing mode:

1. `instant` (default). Blocks a produced automatically for each transaction
2. `<number>`. Blocks are produced once per `number` milliseconds

### Using polkadot-launch

Install [polkadot-launch](https://github.com/paritytech/polkadot-launch).

**Note:** you must have polkadot node `v0.9.11` compiled and built placed in `../polkadot/target/release/`.
To use different localion you can modify `./launch-config.json`.

Build Pontem:

```sh
cd pontem
make build
```

Create keystore path for Pontem:

```sh
mkdir -p ~/.pontem/keystore-1 # Base path
```

Add Nimbus key:

```sh
# Use "//Alice" for URI.
./target/release/pontem key insert --keystore-path ~/.pontem/keystore-1 --key-type nmbs
```

```sh
# run pontem-node
polkadot-launch ./launch-config.json
```

Wait for an minute.

Observe `9946.log` to verify that the node was launched successfully and is producing blocks, also you can use [Web UI](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/explorer).

```sh
tail -f ./9946.log
```

### Using polkadot-launch via docker-compose

Build container:

```sh
cd pontem
docker-compose build
```

Launching services:

```sh
docker-compose up -d
```

Log files are in folder `docker-launch`.

In the `docker-compose.yml` file, you can set the required versions of polkadot and pontem by specifying them in `POLKADOT_VERSION` and `PONTEM_VERSION`, respectively. (note: if you change versions in docker-compose.yaml or change the `.build/launch.Dockerfile`, you need to rerun the `docker-compose build` command).

You can connect using the following ports:

```sh
127.0.0.1:9944 # Alice relaychain
127.0.0.1:9946 # Alice parachain
```

### Manually

Build Polkadot:

```sh
git clone https://github.com/paritytech/polkadot.git
cd polkadot
git fetch origin
git checkout release-v0.9.11
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
cd pontem
make build
```

Create keystore path for Pontem:

```sh
mkdir -p ~/.pontem/keystore-1 # Base path
```

Add Nimbus key:

```sh
# Use "//Alice" for URI.
./target/release/pontem key insert --keystore-path ~/.pontem/keystore-1 --key-type nmbs
```

Launch parachain node as collator:

```sh
./target/release/pontem export-genesis-state --parachain-id 2000 > genesis-state
./target/release/pontem export-genesis-wasm > genesis-wasm
./target/release/pontem --collator --tmp --keystore-path ~/.pontem/keystore-1 --parachain-id 2000 --port 40335 --ws-port 9946 -- --execution wasm --chain ../polkadot/rococo-local-cfde.json --port 30335
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
9. Restart `pontem`-node.

## Metadata

Metadata for Polkadot JS can be found in [repository containing types](https://github.com/pontem-network/pontem-types/blob/main/src/index.ts).

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
./target/release/pontem key insert --keystore-path ~/.pontem/keystore-2 --key-type nmbs
```

Get your public key:

```sh
# Use "//Bob" for dev purposes URI.
./target/release/pontem key inspect
```

You will see something like:

```sh
Secret Key URI `//Bob` is account:
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
/target/release/pontem --collator \
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
/target/release/pontem --collator \
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

### XCM

The XCM implemented includes assets transferring between Parachains and Relaychain using [XTokens](https://github.com/open-web3-stack/open-runtime-module-library/tree/master/xtokens) pallet.

* Transfers from Relaychain to Parachain happens with `reserveTransferAssets`.
* XCM teleport and executions are currently disabled.
* Supports PONT and KSM tokens.

**Dev Relaychain**

In case you want to run node with dev Relaychain (e.g. using `polkadot-launch`), after launching Relaychain send transaction using **sudo** account:

1. Navigate to [sudo](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9946#/sudo).
2. Choose `xcmPallet` pallet.
3. Choose `forceDefaultXcmVersion(maybeXcmVersion)` function.
4. Enable option `maybeXcmVersion`.
5. Put `2` into the `maybeXcmVersion` field.
6. Send transaction.

Now you can use XCM.

## LICENSE

See [LICENSE](/LICENSE).
