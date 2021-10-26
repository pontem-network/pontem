# Multisig Module

**This module is a fork of Substrate FRAME's `multisig` pallet. See below for rationale**

A module for doing multisig dispatch.

- [`multisig::Trait`](https://docs.rs/pallet-multisig/latest/pallet_multisig/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-multisig/latest/pallet_multisig/enum.Call.html)

## Overview

This module contains functionality for multi-signature dispatch, a (potentially) stateful
operation, allowing multiple signed
origins (accounts) to coordinate and dispatch a call from a well-known origin, derivable
deterministically from the set of account IDs and the threshold number of accounts from the
set that must approve it. In the case that the threshold is just one then this is a stateless
operation. This is useful for multisig wallets where cryptographic threshold signatures are
not available or desired.

## Fork

### Reasons

The default `multisig` pallet does not provide a list of signatures which authorized the
transaction, it just provides a proof that such transaction is authorized. There are two
components in this issue:

1. Implementation removes the list of signers from the storage right before the transaction
   execution to avoid reentry attacks.
2. There is no public API to get a list of signers, i.e. a replacement for `ensure_signed`
   function.

### Modifications

1. The pallet updated to FRAMEv2
2. New custom Origin was added to the pallet to store the list of signers. `match` on this
   Origin must be used instead of `ensure_signed` in transactions supporting multisig calls.

### Why not PR

Adding custom Origin breaks backward compatibility of `multisig` pallet and also requires each
user to avoid `ensure_signed` for multisig calls. In turn, each transaction must be modified to
support multisig calls. It seems inconvinient to other users of `multisig`.

## Interface

### Dispatchable Functions

* `as_multi` - Approve and if possible dispatch a call from a composite origin formed from a
  number of signed origins.
* `approve_as_multi` - Approve a call from a composite origin.
* `cancel_as_multi` - Cancel a call from a composite origin.

[`Call`]: ./enum.Call.html
[`Config`]: ./trait.Config.html

License: Apache-2.0
