# Clone move-stdlib
rm -rf ./move-stdlib
git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
pushd ./move-stdlib
git checkout 12c5488729b8377b90f247537459f16ef1383d43
dove build -b
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
pushd ./pont-stdlib
git checkout 1f094231de16cad54f2303093a7f866474bccd12
dove build -b
popd

pushd ./user
dove clean
dove build -b
dove tx "transfer<0x1::NOX::NOX>(Alice, 500000000000)" -o=transfer_pont.mvt
dove tx "transfer<0x1::KSM::KSM>(Alice, 500000000000)" -o=transfer_ksm.mvt
dove tx "deposit_bank<0x1::NOX::NOX>(500000000000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(50000000000000)" -o=deposit_bank_ksm.mvt
popd
