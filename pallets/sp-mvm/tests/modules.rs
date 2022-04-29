/// Tests related to modules/packages publishing.
use frame_support::assert_err_ignore_postinfo;
use frame_support::dispatch::DispatchError;
use sp_runtime::ModuleError;

mod common;
use common::assets::{modules, ROOT_PACKAGE, USER_PACKAGE};
use common::mock::*;
use common::addr::*;
use common::utils;

#[test]
// Publish module as user.
fn publish_module() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();
        utils::publish_module(origin, &modules::user::STORE, None).unwrap();
    });
}

#[test]
/// Check publishing module compiled by one user but published by another one.
fn publish_module_as_wrong_user() {
    RuntimeBuilder::new().build().execute_with(|| {
        let origin = bob_public_key();
        assert_err_ignore_postinfo!(
            utils::publish_module(origin, &modules::root::EVENT_PROXY, None),
            DispatchError::Module(ModuleError {
                index: 6,
                error: 89,
                message: Some("ModuleAddressDoesNotMatchSender")
            })
        );
    });
}

#[test]
/// Publish module as root (should be placed under CORE_CODE_ADDRESS).
fn publish_module_as_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        utils::publish_module_as_root(&modules::root::EVENT_PROXY, None).unwrap();
    });
}

#[test]
/// Publish package as user.
fn publish_package_as_user() {
    RuntimeBuilder::new().build().execute_with(|| {
        let package = &USER_PACKAGE;
        let origin = bob_public_key();

        utils::publish_package(origin, package, None).unwrap();
    });
}

#[test]
/// Publish package as root (should be placed under CORE_CODE_ADDRESS).
fn publish_package_as_root() {
    RuntimeBuilder::new().build().execute_with(|| {
        let package = &ROOT_PACKAGE;

        utils::publish_package_as_root(package, None).unwrap();
    });
}
