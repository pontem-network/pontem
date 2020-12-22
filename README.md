# Substrate Move VM

Substrate node template with Move VM pallet on board.

**It's PoC. Work in progress, so use it at your own risk.**

Current status:

- [X] Run Move scripts by executing transactions
- [X] Use pre-deployed Move modules in your scripts
- [X] A script can accept one U64 argument
- [X] Users can publish modules in dry-run mode
- [ ] Polkadot SS58 addresses doesn't work yet
- [ ] Storage/events also doesn't work yet, so you just run your scripts in dry-run mode


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

## Send script transactions

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

Install **Dove** from [move-tools](https://github.com/dfinance/move-tools) repository, it's Move package-manager, also it contains compiler:

```sh
git clone git@github.com:dfinance/move-tools.git
cd ./move-tools
cargo install --path dove
```

Create first project and script:

```sh
dove new first_project --dialect polkadot
cd ./first_project
touch ./scripts/run.move
```

Put next code inside `run.move`:

```rs
script {
    fun main(a: u64) {
        // A+B
        let b = a + 5;
        let _ = a + b;

        // Loop.
        let i = 0;
        loop {
            i = i + 1;
            if (i == 100) {
                break
            }
        };
    }
}
```

The script will do basic math (A + B) and launch loop, then exit.

Compile script:

```sh
dove build
```

Now see compiled binary at `./target/scripts/0_main.mv`.

Send transaction contains scripts:

1. Navigate to **Developer -> Extrinsics**.
2. Choose `mvm` module.
3. Choose `script_bc` field and click `file upload`.
4. Drag&Drop `./target/scripts/0_main.mv` file there.
5. Choose the `args` field and put there `0x0000000000000001`.
6. Submit a new transaction!

Wait until transaction executed
