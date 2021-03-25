# Substrate Move VM

Substrate node template with Move VM pallet on board.

**It's PoC. Work in progress, so use it at your own risk.**

Current status:

- [X] Run Move scripts by executing transactions.
- [X] Polkadot SS58 addresses support.
- [X] Script transagit ctions support arguments. 
- [X] Users can publish modules under their addresses.
- [X] Publish default (standard) libraries under origin address.
- [X] Writesets processing and storage support.
- [X] Events support.
- [X] Standard library supports native calls, like: block height, timestamp, signatures, etc.

## Installation

We recommend to use [rust toolchain](/rust-toolchain) that we use during development.

Clone current repository:

```sh
git clone git@github.com:pontem-network/sp-move.git
```

Navigate to cloned repository and launch init script:

```sh
cd ./sp-move
./scripts/init.sh
```

Run local node:

```sh
make run
```

## Move VM

To finish the next steps you need the Polkadot address.

Configure UI:

1. After the local node launched visit [UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/settings/developer).
2. Setup UI to use the local node ws://127.0.0.1:9944 if needed.
3. Go to **Settings -> Developer** and put there next JSON (see [issue#78](https://github.com/substrate-developer-hub/substrate-node-template/issues/78) and it also contains types for Move VM events/results):
```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "RawAccountAddress": "[u8;32]",
  "AccountAddress": "[u8;32]",
  "ModuleId": {
    "address": "AccountAddress",
    "name": "Text"
  },
  "TypeTag": {
    "_enum": [
      "Bool",
      "U8",
      "U64",
      "U128",
      "Address",
      "Signer",
      "Vector",
      "Struct"
    ],
    "Bool": null,
    "U8": null,
    "U64": null,
    "U128": null,
    "Address": null,
    "Signer": null,
    "Vector": "Vec<TypeTag>",
    "Struct": "StructTag"
  },
  "StructTag": {
    "address": "AccountAddress",
    "module": "Text",
    "name": "Text",
    "type_params": "Vec<TypeTag>"
  }
}
```
4. Save configuration.

Install **dove** (**Polkadot** version) from [move-tools](https://github.com/pontem-network/move-tools) repository, it's Move package-manager and compiler.

### Deploy standard library.

First of all, we need to deploy and build [Standard Library](https://github.com/pontem-network/move-stdlib):

```sh
git clone git@github.com:pontem-network/move-stdlib.git
cd move-stdlib
dove build
```

See built modules:

```sh
ls -la ./target/modules
```

Now let's deploy modules (**in the same ordering!**):

* `0_Block.mv`
* `5_PONT.mv`
* `7_Signer.mv`
* `8_Event.mv`
* `9_Pontem.mv`
* `10_Account.mv`

Standard library is modules that developers can use by default in Move VM. Usually deploying of standard modules restricted by origin account or by governance. 
Currently, as we still don't support governance, we use origin accounts, in the future we are planning to deploy the initial version of standard library into genesis block in Pontem parachain and update it using governance, but now we support only deploy from origin accounts.

To deploy standard module:

  1. Navigate to **Developer -> Extrinsics**.
  2. Choose the origin account (usually it's Alice).
  3. Choose the `sudo` module.
  4. Choose `sudo(call)` transaction.
  5. Choose the `mvm` module in the `call: Call` field.
  6. Choose `publishStd` transaction.
  7. Click  `addItem`.
  8. For the new field and enable `file upload`.
  9. Upload modules step by step by submitting transactions.
  10. Wait until transactions are confirmed.

Done. At this stage we deployed a standard library. Once transactions successfully executed, check 'Explorer', there will be latest events generated by our transactions.

### MINT token and first transfers.

Let's mint new PONT tokens using script and transfer it to another account.

First of all, create new dove project:

```sh
dove new first_project --dialect polkadot --address <ss58 address>
cd ./first_project
```

Replace `<ss58 address>` with your Polkadot address.

Create `mint_and_transfer` script:

```sh
touch ./scripts/mint_and_transfer.move
```

Put next code inside `./scripts/mint_and_transfer.move` (see comments for details):

```rs
script {
    // Use standard library.
    use 0x01::PONT;
    use 0x01::Pontem;
    use 0x01::Account;

    // Accepts 3 arguments. First argument is the default and sender account, so you can ignore it.
    // Arguments:
    // * amount_to_send - how much of minted tokens send to another account.
    // * amount_to_keep - how much of minted tokens keep on the sender account.
    // * payee - recipient, address of account where we are going to send PONT tokens.
    fun mint_and_send(sender: &signer, amount_to_send: u128, amount_to_keep: u128, payee: address) {
        // Register PONT coin with denom "PONT" and 18 decimals.
        // Usually this function is restricted, but for demo proposals we keep it opened for anyone for now.
        Pontem::register_coin<PONT::T>(b"504f4e54", 18);

        // Mint new PONT tokens. We will get balanced resources that we can use later.
        // Usually this function is restricted, but for demo proposals we keep it opened for anyone for now.
        let balance = Pontem::mint<PONT::T>(amount_to_send + amount_to_keep);      

        // Deposit minted tokens to the sender account.
        Account::deposit_to_sender<PONT::T>(sender, balance);

        // Send part of minted tokens to payee account.
        Account::pay_from_sender<PONT::T>(sender, payee, amount_to_send);

        // Check the new balance of the sender, if wrong balance - throw error 101.
        assert(Account::balance<PONT::T>(sender) == amount_to_keep, 101);

        // Check the new balance of payee, if wrong balance - throw error 102.
        assert(Account::balance_for<PONT::T>(payee) == amount_to_send, 102);
    }
}
```

As you see script mint new PONT tokens, deposit it to sender account and transfer to payee, after checking balances with assert.

Let's compile script and provide arguments to get binary contains arguments and script.

```sh
dove build
```

If compiled correctly, without errors, you are doing all right.

To send transactions we need to compile the script together with arguments, special for this we have `dove ct` command, dove will take your script and arguments and make a new binary with mvt extension, that you can use to send transactions. You can read more about this command in dove [documentation](https://github.com/pontem-network/move-tools#arguments).

We will prepare transaction from Bob's account, and Alice's account will receive new minted tokens, so copy-paste alice address and use in next command:

```sh
dove ct 'mint_and_send(150000000000000000000,200000000000000000000,5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)'
```

Dove will generate new file contains both scripts and arguments, check it:

```sh
ls -la ./target/transactions
```

You will see `mint_send.mvt`.

Send script transaction:

1. Navigate to **Developer -> Extrinsics**.
2. Choose `mvm` module.
3. Choose the correct account. 
4. Choose `execute` transaction.
5. Choose `script_bc` field and enable `file upload`.
6. Upload `./target/transactions/mint_send.mvt`.
7. Submit a new transaction!
8. Wait until the transaction is confirmed.

Once the transaction successfully executed, check 'Explorer', there will be latest events generated by our transactions.

### View resources.

Time to view resources we generated during the latest script. It will get balances for both accounts and information about PONT token, that we minted in the script.

First of all let's install `move-resource-viewer`:

```sh
cargo install --git https://github.com/pontem-network/move-tools.git move-resource-viewer \
    --no-default-features \                                               
    --features="json-schema, ps_address"
```

Let's see help and check installation:

```sh
move-resource-viewer help
```

The command prints documentation, and if all fine, you will see options. Also, you can look at resource viewer documentation on [Github](https://github.com/pontem-network/move-tools/tree/master/resource-viewer).

Query user balance (replace address with yours):

```sh
move-resource-viewer -a 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty -q "0x1::Account::Balance<0x1::PONT::T>" --api="ws://127.0.0.1:9944"  -o=output.json
cat ./output.json
```

You will get something like (showing your PONT token balance and other related information):

```json
{
  "height": "0xc8d0a7ed6995aa3172002d65336ea969dd10e5855a64db4c9e4dd00c6b2c469f",
  "result": {
    "is_resource": true,
    "type": {
      "address": [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1
      ],
      "module": "Account",
      "name": "Balance",
      "type_params": [
        {
          "Struct": {
            "address": "0000000000000000000000000000000000000000000000000000000000000001",
            "module": "PONT",
            "name": "T",
            "type_params": []
          }
        }
      ]
    },
    "value": [
      {
        "id": "coin",
        "value": {
          "Struct": {
            "is_resource": true,
            "type": {
              "address": [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1
              ],
              "module": "Pontem",
              "name": "T",
              "type_params": [
                {
                  "Struct": {
                    "address": "0000000000000000000000000000000000000000000000000000000000000001",
                    "module": "PONT",
                    "name": "T",
                    "type_params": []
                  }
                }
              ]
            },
            "value": [
              {
                "id": "value",
                "value": {
                  "U128": 200000000000000000000
                }
              }
            ]
          }
        }
      }
    ]
  }
}
```

Same here you can do with any other resource, for example you can deploy a new module:

```rs
module Store {
    resource struct U64 {val: u64}
    resource struct Address {val: address}
    resource struct VectorU8 {val: vector<u8>}

    public fun store_u64(account: &signer, val: u64) {
        let foo = U64 {val: val};
        move_to<U64>(account, foo);
    }

    public fun store_address(account: &signer, val: address) {
        let addr = Address {val: val};
        move_to<Address>(account, addr);
    }

    public fun store_vector_u8(account: &signer, val: vector<u8>) {
        let vec = VectorU8 {val: val};
        move_to<VectorU8>(account, vec);
    }
}
```

Store U64 number inside resource using script (replace dependency address with yours):

```rs
script {
    use 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;
    
    fun store_num(account: &signer, a: u64) {
        Store::store_u64(account, a);
    }
}
```

Query stored resource:

```sh
move-resource-viewer -a 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty -q "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store::U64" --api="ws://127.0.0.1:9944"  -o=output.json
```

You will get something like:

```json
{
  "height": "0xc02a5689b6120fa495e6a5606b279c7a4b200f2fb5e4d0edec6969578d81ecef",
  "result": {
    "is_resource": true,
    "type": {
      "address": [
        142,
        175,
        4,
        21,
        22,
        135,
        115,
        99,
        38,
        201,
        254,
        161,
        126,
        37,
        252,
        82,
        135,
        97,
        54,
        147,
        201,
        18,
        144,
        156,
        178,
        38,
        170,
        71,
        148,
        242,
        106,
        72
      ],
      "module": "Store",
      "name": "U64",
      "type_params": []
    },
    "value": [
      {
        "id": "val",
        "value": {
          "U64": 10
        }
      }
    ]
  }
}
```

### Fetch block height.

Create another script, let's call it block and put following script inside:

```rs
script {
    use 0x01::Event;
    use 0x01::Block;

    fun block(account: &signer) {
        // Get the latest block height.
        let current_height = Block::get_current_block_height();

        // Emit event with the current height.
        Event::emit(account, current_height);
    }
}
```

Current script fetch latest block height and emit event.

Build script:

```sh
dove ct 'block()'
```

Send script transaction:

1. Navigate to **Developer -> Extrinsics**.
2. Choose `mvm` module.
3. Choose the correct account. 
4. Choose `execute` transaction.
5. Choose `script_bc` field and enable `file upload`.
6. Upload `./target/transactions/block.mvt`.
7. Submit a new transaction!
8. Wait until the transaction is confirmed.

Once the transaction successfully executed, check 'Explorer', there will be latest events generated by our transactions. 

Look at more examples in [tests](pallets/sp-mvm/tests/).
Also, read our [Move Book](https://move.pontem.network) to learn Move language and find more examples.

## LICENSE

See [LICENSE](/LICENSE).
