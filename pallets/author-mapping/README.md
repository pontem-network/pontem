# Author mapping pallet

**This pallet is a copy from [Moonbeam](https://github.com/PureStake/moonbeam) project**

## Overview

Maps Author Ids as used in Nimbus consensus layer to account ids as used in the runtime.
This should likely be moved to Nimbus eventually.

This pallet maps AuthorId => AccountId which is most useful when using propositional style
queries. This mapping will likely need to go the other way if using exhaustive authority sets.
That could either be a seperate pallet, or this pallet could implement a two-way mapping. But
for now it is one-way.

## Modifications

TODO

## License

Licensed under GNU General Public License version 3.

