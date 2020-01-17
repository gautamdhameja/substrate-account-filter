# Substrate Account Set

A Substrate pallet for account-level permissioning.

The pallet maintains a whitelist of accounts that are permitted to submit extrinsics, and allows the sudo user to add and remove accounts from this whitelist.

The filtering of incoming transactions is done by implementing the `SignedExtension` trait.

## Usage

* Add the module's dependency in the `Cargo.toml` of your `runtime` directory. Make sure to enter the correct path or git url of the pallet as per your setup.

```toml
[dependencies.substrate_account_set]
package = 'substrate-account-set'
git = 'https://github.com/gautamdhameja/substrate-account-set.git'
default-features = false
```

* Declare the pallet in your `runtime/src/lib.rs`.

```rust
pub use substrate_account_set;

impl substrate_account_set::Trait for Runtime {
    type Event = Event;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        ...
        ...
        ...
        AccountSet: substrate_account_set::{Module, Call, Storage, Event<T>, Config<T>},
    }
);
```

* Add the module's `WhitelistAccount` type in the `SignedExtra` checklist.

```rust
pub type SignedExtra = (
    ...
    ...
    balances::TakeFees<Runtime>,
    substrate_account_set::WhitelistAccount<Runtime>
```

* Add a genesis configuration for the module in the `src/chain_spec.rs` file. This configuration adds the initial account ids to the account whitelist.

```rust
    substrate_account_set: Some(AccountSetConfig {
        whitelisted_accounts: vec![(get_account_id_from_seed::<sr25519::Public>("Alice"), true),
            (get_account_id_from_seed::<sr25519::Public>("Bob"), true)],
    }),
```

* `cargo build --release` and then `cargo run --release -- --dev`

When the node starts, only the account ids added in the genesis config of this module will be able to send extrinsics to the runtime. This means that you **should not leave the genesis config empty** or else no one will be able to submit any extrinsics.

New `AccountId`s can be added to the whitelist by calling the pallet's `add_account` function using `root` key as origin.

## Sample

The usage of this pallet are demonstrated in the [Substrate permissioning sample](https://github.com/gautamdhameja/substrate-permissioning).

## Potential extension:

* The addition and removal of account id's to the whitelist could also be done using other governance methods instead of root.
* The logic can be reversed to maintain a blacklist of accounts which cannot send extrinsics to the runtime.

## Disclaimer

This code not audited and reviewed for production use cases. You can expect bugs and security vulnerabilities. Do not use it as-is in real applications.
