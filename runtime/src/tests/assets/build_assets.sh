# Clone move-stdlib
rm -rf ./move-stdlib
git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
pushd ./move-stdlib
git checkout ccd25dfc85c812f56b4a7120bce793edd5f19064
dove build -b
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
pushd ./pont-stdlib
git checkout 0702cdf5d696bc50b366e04de1b59ccc3d904032
dove build -b
popd

pushd ./user
dove clean
dove build -b
dove tx "transfer<0x1::PONT::PONT>(Alice, 500000000000)" -o=transfer_pont.mvt
dove tx "transfer<0x1::KSM::KSM>(Alice, 500000000000)" -o=transfer_ksm.mvt
dove tx "deposit_bank<0x1::PONT::PONT>(500000000000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(50000000000000)" -o=deposit_bank_ksm.mvt
popd
