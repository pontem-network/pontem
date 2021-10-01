# MVM pallet

## Overview

This pallet provides [Move virtual machine](https://github.com/pontem-network/sp-move-mv) to execute Move smart-contracts on
Substrate chain.

## API

 - `execute(tx_bc: Vec<u8>, gas_limit: u64)` - execute transaction with bytecode `tx_bc`
 - `publish_module(module_bc: Vec<u8>, gas_limit: u64)` - publish Move module with bytecode `module_bc`
 - `publish_package(package: Vec<u8>, gas_limit: u64)` - publish package (a set of modules) from binary `package`
 - `publish_std(modules: Vec<Vec<u8>>, gas_limit: u64)` - publish a list of std modules (only callable by root)

 ## LICENSE

 Licensed under the Apache License, Version 2.0
