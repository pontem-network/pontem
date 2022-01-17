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
dove build --package
dove tx "transfer<0x1::PONT::PONT>(Alice, 500000000000)" -o=transfer_pont.mvt
dove tx "transfer<0x1::KSM::KSM>(Alice, 500000000000)" -o=transfer_ksm.mvt
dove tx "deposit_bank<0x1::PONT::PONT>(500000000000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(50000000000000)" -o=deposit_bank_ksm.mvt
popd
