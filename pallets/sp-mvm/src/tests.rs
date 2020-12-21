use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        let code = vec![42, 0, 0, 0];
        let key = vec![0, 0, 0, 0];
        // Dispatch a signed extrinsic.
        assert_ok!(Mvm::execute(Origin::signed(1), code));
        // Read pallet storage and assert an expected result.
        assert_eq!(Mvm::vmstorage(&key), Some(vec![42]));
    });
}

// #[test]
// fn correct_error_for_none_value() {
//     new_test_ext().execute_with(|| {
//         // Ensure the expected error is thrown when no value is present.
//         assert_noop!(
//             Mvm::cause_error(Origin::signed(1)),
//             Error::<Test>::NoneValue
//         );
//     });
// }
