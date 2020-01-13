#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use codec::{Decode, Encode};
use sp_std::marker::PhantomData;
use sp_std::fmt::Debug;
use frame_support::{
    decl_event, decl_storage, decl_module,
    dispatch,
    weights::{DispatchInfo},
};
use system::{self as system, ensure_root};
use sp_runtime::{
    transaction_validity::{
		ValidTransaction, TransactionValidityError,
        InvalidTransaction, TransactionValidity,
        TransactionPriority, TransactionLongevity,
	},
    traits::{SignedExtension}
};

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as AccountSet {
        WhitelistedAccounts get(whitelisted_accounts) config(): map T::AccountId => ();
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        /// Add a new account to the whitelist.
        /// Can only be called by the root.
        pub fn add_account(origin, new_account: T::AccountId) -> dispatch::DispatchResult {
            ensure_root(origin)?;

            <WhitelistedAccounts<T>>::insert(&new_account, ());

            Self::deposit_event(RawEvent::AccountWhitelisted(new_account));

            Ok(())
        }

        /// Remove an account from the whitelist.
        /// Can only be called by the root.
        pub fn remove_account(origin, account_to_remove: T::AccountId) -> dispatch::DispatchResult {
            ensure_root(origin)?;

            <WhitelistedAccounts<T>>::remove(&account_to_remove);

            Self::deposit_event(RawEvent::AccountRemoved(account_to_remove));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // When a new account is added to the whitelist.
        AccountWhitelisted(AccountId),
        // When an account is removed from the whitelist.
        AccountRemoved(AccountId),
    }
);

/// The following section of the code implements the `SignedExtension` trait
/// for the `WhitelistAccount` type.
/// `SignedExtension` is being used here to filter out the non-whitelisted accounts
/// when they try to send extrinsics to the runtime.
/// Inside the `validate` function of the `SignedExtension` trait,
/// we check if the sender (origin) of the extrinsic is part of the 
/// whitelist or not.
/// The extrinsic will be rejected as invalid if the origin is not part 
/// of the whitelist.

/// The `WhitelistAccount` struct.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct WhitelistAccount<T: Trait + Send + Sync>(PhantomData<T>);

/// Debug impl for the `WhitelistAccount` struct.
impl<T: Trait + Send + Sync> Debug for WhitelistAccount<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "WhitelistAccount")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

/// Implementation of the `SignedExtension` trait for the `WhitelistAccount` struct.
impl<T: Trait + Send + Sync> SignedExtension for WhitelistAccount<T> {
    type AccountId = T::AccountId;
    type AdditionalSigned = ();
    type DispatchInfo = DispatchInfo;
    type Call = T::Call;
    type Pre = ();
    
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> { Ok(()) }

    // Filter out the non-whitelisted keys.
    // If the key is in the whitelist, return a valid transaction,
    // else return a custom error.
    fn validate(
        &self,
        who: &Self::AccountId,
        _call: &Self::Call,
        info: DispatchInfo,
        _len: usize,
    ) -> TransactionValidity {
        if <WhitelistedAccounts<T>>::exists(who) {
            Ok(ValidTransaction {
                priority: info.weight as TransactionPriority,
                longevity: TransactionLongevity::max_value(),
                propagate: true,
                ..Default::default()
            })
        } else {
            Err(InvalidTransaction::Call.into())
        }
    }
}
