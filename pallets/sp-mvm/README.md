# MVM pallet

## Overview

This pallet provides [Move virtual machine](https://github.com/pontem-network/sp-move-mv) to execute Move smart-contracts on
Substrate chain.

## API

All provided extrinsics functions require to configure a gas limit, similar to EVM.

 - `execute(tx_bc: Vec<u8>, gas_limit: u64)` - execute Move script with bytecode `tx_bc`.
 - `publish_module(module_bc: Vec<u8>, gas_limit: u64)` - publish Move module with bytecode `module_bc`.
 - `publish_package(package: Vec<u8>, gas_limit: u64)` - publish package (a set of Move modules) from binary `package`. Allows to update Standard Library if calls from root, in the future root will be replaced with gov.
 - `publish_std(modules: Vec<Vec<u8>>, gas_limit: u64)` - batch publish Move Standard Library modules by root account only. Not recommended to use, would be deprecated.

Read more about the Move VM pallet in the [Pontem Documentation](https://docs.pontem.network/03.-move-vm/move_vm).

 ## LICENSE

 Licensed under the Apache License, Version 2.0
