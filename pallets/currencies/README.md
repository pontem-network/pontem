# Currencies Module

This module initially forked from [Acala Network](https://github.com/AcalaNetwork/Acala), [revision](https://github.com/AcalaNetwork/Acala/commit/5715d9374ea3e147937e9b0384ecddea0f4e616d).

Changes done so far:

* Now the `MultiCurrency` parameter requires `frame_support::traits::tokens::fungibles` trait.
* Now the `NativeCurrency` parameter requires `frame_support::traits::tokens::fungible` trait.
* Implemented `frame_support::traits::tokens::fungibles` for `Pallet<T>`.
* Removed `EVM` dependencies and functionality.

## Overview

The currencies module provides a mixed currencies system, by configuring a native currency which implements `BasicCurrencyExtended`, and a multi-currency which implements `MultiCurrency`.

It also provides an adapter, to adapt `frame_support::traits::Currency` implementations into `BasicCurrencyExtended`.

The currencies module provides functionality of both `MultiCurrencyExtended` and `BasicCurrencyExtended`, via unified interfaces, and all calls would be delegated to the underlying multi-currency and base currency system. A native currency ID could be set by `Trait::GetNativeCurrencyId`, to identify the native currency.

## License

[LICENSE](./LICENSE)
