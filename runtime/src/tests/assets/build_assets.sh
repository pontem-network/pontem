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
dove tx "transfer<0x1::PONT::PONT>(treasury, gkQ5K6EnLRgZkwozG8GiBAEnJyM6FxzbSaSmVhKJ2w8FcK7ih, 500000000000)" -o=transfer_pont.mvt
dove tx "transfer<0x1::KSM::KSM>(treasury, gkQ5K6EnLRgZkwozG8GiBAEnJyM6FxzbSaSmVhKJ2w8FcK7ih, 500000000000)" -o=transfer_ksm.mvt
dove tx "deposit_bank<0x1::PONT::PONT>(500000000000)" -o=deposit_bank_pont.mvt
dove tx "deposit_bank<0x1::KSM::KSM>(50000000000000)" -o=deposit_bank_ksm.mvt
popd
