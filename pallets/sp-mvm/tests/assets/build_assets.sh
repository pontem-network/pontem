# Clone and rebuild stdlib
rm -rf ./stdlib
git clone https://github.com/pontem-network/move-stdlib.git ./stdlib
pushd ./stdlib
git checkout 7f200e70d2fc98863ecc8e004e9ee2255e97bf5a
dove build --package
popd

pushd ./user
dove clean
dove build --tree
dove build --package
dove tx "store_u64(42)"
dove tx "emit_event(42)"
dove tx "store_system_block()"
dove tx "store_system_timestamp()"
dove tx "inf_loop()"
dove tx "store_native_balance()"
dove tx "transfer(treasury, 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, 2000)"
dove tx "multisig_test()"
#dove tx "store_native_deposit(false)"
#dove tx "store_native_deposit(true)" -o=store_native_deposit_reg
#dove tx "store_native_withdraw(false)"
#dove tx "store_native_withdraw(true)" -o=store_native_withdraw_reg
#dove tx "missed_native_balance()"
#dove tx "get_price_test()"
popd

pushd ./root
dove clean
dove build
dove build --package
pushd
