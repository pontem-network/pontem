# Clone move-stdlib
rm -rf ./move-stdlib
git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
pushd ./move-stdlib
git checkout 79ed97fc1f98fefab16fbb54988bdc7defb09578
dove build --package
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
pushd ./pont-stdlib
git checkout aa3dcdd5ed62b8912e0f95108ca1451162d385ac
dove build --package
popd

pushd ./user
dove clean
dove build
dove build --package
dove tx "store_u64(42)"
dove tx "emit_event(42)"
dove tx "store_system_block()"
dove tx "store_system_timestamp()"
dove tx "inf_loop()"
dove tx "store_native_balance()"
dove tx "store_token_balance()"
dove tx "transfer<0x1::PONT::PONT>(Alice, 2000)"
dove tx "transfer<0x1::KSM::KSM>(Alice, 2000)"
dove tx "multisig_test()"
dove tx "deposit_bank<0x1::PONT::PONT>(2000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(2000)" -o=deposit_bank_ksm.mvt
popd

pushd ./root
dove clean
dove build
dove build --package
pushd
