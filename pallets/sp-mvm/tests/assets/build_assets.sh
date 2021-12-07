# Clone and rebuild stdlib
rm -rf ./stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./stdlib
pushd ./stdlib
git checkout e9bd26720c06705d2e222833a496fda7c67c8e32
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
dove tx "store_token_balance()"
dove tx "transfer(treasury, 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, 2000)"
dove tx "transfer_token(treasury, 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, 2000)"
dove tx "multisig_test()"
dove tx "deposit_bank<0x1::PONT::PONT>(2000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(2000)" -o=deposit_bank_ksm.mvt
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
