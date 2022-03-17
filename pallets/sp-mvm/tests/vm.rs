/// Tests related to VM usage and management.
mod common;
use common::assets::modules;
use common::mock::*;
use common::addr::*;
use common::utils;

#[test]
/// Cache should be cleaned for new block if VM used.
fn mvm_cleanup_cache() {
    use sp_mvm::mvm::MoveVmUsed;

    RuntimeBuilder::new().build().execute_with(|| {
        roll_next_block();
        // on finalize -> clean cache
        assert!(!sp_mvm::Pallet::<Test>::is_move_vm_used());

        // use the VM
        utils::publish_module(bob_public_key(), &modules::user::STORE, None).unwrap();

        // VM used just now
        assert!(sp_mvm::Pallet::<Test>::is_move_vm_used());

        roll_next_block();
        // on finalize -> clean cache
        assert!(!sp_mvm::Pallet::<Test>::is_move_vm_used());
    });
}
