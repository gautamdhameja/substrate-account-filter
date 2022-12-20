mod mock;

use crate::mock::{test, RuntimeOrigin, Test, AccountFilter, Balances};
use frame_support::{assert_noop, assert_ok, pallet_prelude::*};
use substrate_account_filter::{Error, Event};

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
        assert!(matches!(AccountFilter::allowed_accounts(2u64), Some(_)));
        assert_ok!(Balances::transfer(RuntimeOrigin::signed(2), 1, 10));
    });
}