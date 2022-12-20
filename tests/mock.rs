use substrate_account_filter as account_filter;

use frame_support::{parameter_types, traits::GenesisBuild};
use frame_system::EnsureRoot;

use sp_runtime::{
	impl_opaque_keys,
	testing::{Header, UintAuthorityId},
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::H256;
use sp_std::convert::{TryFrom, TryInto};

impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub dummy: UintAuthorityId,
	}
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test, (), SignedExtra>;
type Block  = sp_runtime::generic::Block<sp_runtime::generic::Header<<Test as frame_system::Config>::BlockNumber, sp_runtime::traits::BlakeTwo256>, UncheckedExtrinsic>;
type SignedExtra = (
    account_filter::AllowAccount<Test>,
);
type Balance = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		AccountFilter: account_filter,
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},

	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(frame_support::weights::Weight::from_ref_time(1024));
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl account_filter::Config for Test {
	type ValidateOrigin = EnsureRoot<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 10;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = frame_support::traits::ConstU64<10>;
	type AccountStore = System;
	type MaxLocks = ();
	type WeightInfo = ();
	type MaxReserves = frame_support::traits::ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

pub fn test() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    account_filter::GenesisConfig::<Test> {
        allowed_accounts: vec![(1, ()).into(), (2, ()).into()]
    }.assimilate_storage(&mut t).unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances:
            vec![
                (1, 100),
                (2, 100),
                (3, 100),
                (4, 100)
            ]
    }
    .assimilate_storage(&mut t)
    .unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}