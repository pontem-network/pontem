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

Read [official documentation](https://docs.pontem.network/02.-getting-started/local_node).

## Register PONT coin

We need to register PONT coin information, so create new project using dove and write new script:

```rustc
script {
    use 0x1::PONT;
    use 0x1::Pontem;

    fun register_pont() {
        // To make sure PONT coin registered and known.
        Pontem::register_coin<PONT::T>(b"PONT", 6);
    }
}
```

Compile transaction script:

```sh
dove ct 'register_pont()'
```

Execute script using [UI](./ui.md) or [CLI](./cli.md).

## More

See [Move VM Pallet documentation](https://docs.pontem.network/02.-getting-started/getting_started).

## LICENSE

See [LICENSE](/LICENSE).
