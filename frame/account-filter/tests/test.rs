mod mock;

use crate::mock::{test, RuntimeOrigin, Test, AccountFilter, CALL};
use frame_support::{assert_noop, assert_ok, pallet_prelude::*, dispatch::DispatchInfo};
use substrate_account_filter::{Error, Event};
use sp_runtime::traits::SignedExtension;

#[test]
fn default_test() {
    test().execute_with(|| {
        assert!(matches!(AccountFilter::allowed_accounts(1u64), Some(_)));
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));
    });
}


#[test]
fn test_adding() {
    test().execute_with(|| {
        assert_eq!(AccountFilter::allowed_accounts(4u64), None);
        assert_ok!(AccountFilter::add_account(RuntimeOrigin::root(), 4));
        mock::System::assert_has_event(Event::AccountAllowed(4u64).into());
        assert!(matches!(AccountFilter::allowed_accounts(4u64), Some(_)));

    });
}

#[test]
fn removed_account() {
    test().execute_with(|| {
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));
        assert_ok!(AccountFilter::remove_account(RuntimeOrigin::root(), 2));
        mock::System::assert_has_event(Event::AccountRemoved(2u64).into());
        assert_eq!(AccountFilter::allowed_accounts(2u64), None);
    });
}

#[test]
fn failure_to_add_with_bad_origin() {
    test().execute_with(|| {
        assert_noop!(AccountFilter::add_account(RuntimeOrigin::signed(1), 4), DispatchError::BadOrigin);
    });
}

#[test]
fn failure_to_remove_with_bad_origin() {
    test().execute_with(|| {
        assert_noop!(AccountFilter::remove_account(RuntimeOrigin::signed(1), 2), DispatchError::BadOrigin);
    });
} 

#[test]
fn duplicate_adding_failure() {
    test().execute_with(|| {
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));
        assert_noop!(AccountFilter::add_account(RuntimeOrigin::root(), 2u64), Error::<Test>::Duplicate);
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));

    });
}

#[test]
fn removing_failure() {
    test().execute_with(|| {
        assert_eq!(AccountFilter::allowed_accounts(4u64), None);
        assert_noop!(AccountFilter::remove_account(RuntimeOrigin::root(), 4), Error::<Test>::AccountNotAdded);
        assert_eq!(AccountFilter::allowed_accounts(4u64), None);
    });
}

#[test]
fn send_transfer_success() {
    test().execute_with(|| {
        let info = DispatchInfo::default();
		let len = 0_usize;
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));
        assert_ok!(substrate_account_filter::AllowAccount::<Test>::new().validate(&2, &CALL, &info, len));
    });
}

#[test]
fn send_transfer_failure() {
    let mut ext = test();
    ext.execute_with(|| {
        let info = DispatchInfo::default();
		let len = 0_usize;
        assert_eq!(AccountFilter::allowed_accounts(3u64), None);
        assert_noop!(substrate_account_filter::AllowAccount::<Test>::new().validate(&3, &CALL, &info, len), InvalidTransaction::BadSigner);
    });

    ext.commit_all().unwrap();
}

#[test]
fn send_success_after_adding_account() {
    let mut ext = test();
    ext.execute_with(|| {
        let info = DispatchInfo::default();
		let len = 0_usize;
        assert_eq!(AccountFilter::allowed_accounts(3u64), None);
        assert_noop!(substrate_account_filter::AllowAccount::<Test>::new().validate(&3, &CALL, &info, len), InvalidTransaction::BadSigner);
        assert_ok!(AccountFilter::add_account(RuntimeOrigin::root(), 3));
        assert_ok!(substrate_account_filter::AllowAccount::<Test>::new().validate(&3, &CALL, &info, len));
    });

    ext.commit_all().unwrap();
}

#[test]
fn send_fails_after_removing() {
    let mut ext = test();
    ext.execute_with(|| {
        let info = DispatchInfo::default();
		let len = 0_usize;
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));
        assert_ok!(substrate_account_filter::AllowAccount::<Test>::new().validate(&2, &CALL, &info, len));
        assert_ok!(AccountFilter::remove_account(RuntimeOrigin::root(), 2));
        assert_noop!(substrate_account_filter::AllowAccount::<Test>::new().validate(&2, &CALL, &info, len), InvalidTransaction::BadSigner);
    });

    ext.commit_all().unwrap();
}