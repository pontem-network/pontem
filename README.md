# Substrate Move VM

Substrate node template with Move VM pallet on board.

**It's PoC. Work in progress, so use it at your own risk.**

Current status:

- [X] Run Move scripts by executing transactions
- [X] Use pre-deployed Move modules in your scripts
- [X] A script can accept only U64 arguments currently
- [X] Users can publish modules in dry-run mode
- [X] Polkadot SS58 addresses support
- [x] Storage/events alpha version currently, so you just run your scripts in dry-run mode

## Installation

Clone current repository:

```sh
git clone git@github.com:dfinance/sp-move.git
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

To finish next steps you need Polkadot address.

Configure UI:

1. After the local node launched visit [UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/settings/developer).
2. Setup UI to use the local node ws://127.0.0.1:9944 if needed.
3. Go to **Settings -> Developer** and put there next JSON (see [issue#78](https://github.com/substrate-developer-hub/substrate-node-template/issues/78)):
```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId"
}
```
4. Save configuration.

Install **dove** (**Polkadot** version) from [move-tools](https://github.com/dfinance/move-tools) repository, it's Move package-manager and compiler.

Create first project and store module:

```sh
dove new first_project --dialect polkadot --address <ss58 address>
cd ./first_project
```

Replace `<ss58 address>` with your Polkadot address.

Create `Store` module:

```sh
touch ./modules/store.move
```

Put next code inside `./modules/store.move`:

```rs
module Store {
    resource struct U64 {val: u64}

    public fun store_u64(account: &signer, val: u64) {
        let foo = U64 {val: val};
        move_to<U64>(account, foo);
    }
}
```

The module stores U64 number as resource under sender account.

Now create script:

```sh
touch ./scripts/main.move
```

The script will store U64 value under sender account.

Put next code inside `./scripts/main.move`:

```rs
script {
    use <ss58 address>::Store;

    fun main(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}
```

Replace `<ss58 address>` with your Polkadot SS58 address, e.g. use `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty::Store;`

Compile both module and script:

```sh
dove build
```

Now see compiled binary at `./target/scripts/0_main.mv` and `./target/modules/0_Store.mv`.

Deploy the module via UI:

1. Navigate to **Developer -> Extrinsics**.
2. Choose correct account.
3. Choose `mvm` module.
4. Choose `publishModule` transaction.
5. Choose `module_bc` field and enable `file upload`.
6. Upload `./target/modules/0_Store.mv`.
7. Submit a new transaction!
8. Wait until transaction confirmed.

The module deployed, it's time to execute the script:

1. Choose `mvm` module.
2. Choose correct account. 
3. Choose `execute` transaction.
4. Choose `script_bc` field and enable `file upload`.
5. Upload `./target/scripts/0_main.mv`.
6. Choose `args` field and enable `include option`.
7. Click `Add item`.
8. Put value into a new field, e.g. `1000`.
9. Submit a new transaction!
10. Wait until transaction confirmed.

Congrats! You deployed first module, executed script and stored a value.

Look at more examples in [tests](pallets/sp-mvm/tests/).
