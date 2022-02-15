# Clone move-stdlib
rm -rf ./move-stdlib
git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
pushd ./move-stdlib
git checkout release-v1.0.0
dove build -b
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
pushd ./pont-stdlib
git checkout release-v1.0.0
dove build -b
popd

pushd ./user
dove clean
dove build
dove build -b
dove tx "store_u64(42)"
dove tx "emit_event(42)"
dove tx "store_system_block()"
dove tx "store_system_timestamp()"
dove tx "inf_loop()"
dove tx "store_native_balance()"
dove tx "store_token_balance()"
dove tx "as_root(dr)"
dove tx "transfer<0x1::NOX::NOX>(Alice, 2000)" -o=transfer.mvt
dove tx "transfer<0x1::KSM::KSM>(Alice, 2000)" -o=transfer_token.mvt
dove tx "multisig_test()"
dove tx "deposit_bank<0x1::NOX::NOX>(2000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(2000)" -o=deposit_bank_ksm.mvt
dove tx "signer_one()" -o=signer_user.mvt
dove tx "signer_one(root)" -o=signer_root.mvt
popd

pushd ./root
dove clean
dove build
dove build -b
pushd
