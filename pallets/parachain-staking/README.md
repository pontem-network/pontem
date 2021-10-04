# DPoS Pallet for Parachain Staking

**This pallet is a copy from [Moonbeam](https://github.com/PureStake/moonbeam) project**

## LICENSE

Licensed under GNU General Public License version 3.

## Modifications

No modifications were made at the moment. 
Used commit [`ac7b649e2fcab`](https://github.com/PureStake/moonbeam/tree/ac7b649e2fcabde78d86db0a37445c2838d6ea85).

## Formatting Rules

- dependencies in alphabetical order in the `Cargo.toml` and at the top of each file
- prefer explicit imports to glob import syntax i.e. prefer `use::crate::{Ex1, Ex2, ..};` to `use super::*;`

## Description

Implements Delegated Proof of Stake to

1. select the active set of eligible block producers
2. reward block authors
3. enable nominators and collators to participate in inflationary rewards

Links:

- [Rust Documentation](https://purestake.github.io/moonbeam/parachain_staking/index.html)
- [Unofficial Documentation](https://meta5.world/parachain-staking-docs/)
- [(Outdated) Blog Post with Justification](https://meta5.world/posts/parachain-staking)

## History

Since January 2021, Moonbeam's team has maintained this Delegated Proof of Stake (DPoS) pallet designed specifically for parachains.

Since April 2021, the development of this pallet has been supported by [a Web3 Foundation grant](https://github.com/w3f/Grants-Program/pull/389). The [first milestone](https://github.com/w3f/Grant-Milestone-Delivery/pull/218) was approved in June 2021.
