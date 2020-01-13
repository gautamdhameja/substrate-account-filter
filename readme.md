# Substrate Account Set

A Substrate pallet for account-level permissioning.

The pallet maintains a whitelist of accounts and allows the sudo user to add/remove accounts from this whitelist.

The filtering of incoming transactions is done by implementing the `SignedExtension` trait.

## Usage

* Add the module's dependency in the `cargo.toml` of your `runtime` directory. Make sure to enter correct path or git url of the module as per your setup.

```toml
[dependencies.accountset]
package = 'substrate-account-set'
git = 'https://github.com/gautamdhameja/substrate-account-set.git'
default-features = false
```

* Declare the module in your `runtime/src/lib.rs`.

```rust
pub use accountset;

impl accountset::Trait for Runtime {
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
        AccountSet: accountset::{Module, Call, Storage, Event<T>, Config<T>},
    }
);
```

* Add the module's `WhitelistAccount` type in the `SignedExtra` checklist.

```rust
pub type SignedExtra = (
    ...
    ...
    balances::TakeFees<Runtime>,
    accountset::WhitelistAccount<Runtime>
```

* Add genesis configuration for the module in the `src/chain_spec.rs` file. This configuration adds the initial account ids to the account whitelist.

```rust
    accountset: Some(AccountSetConfig {
        whitelisted_accounts: vec![(get_account_id_from_seed::<sr25519::Public>("Alice"), ()),
            (get_account_id_from_seed::<sr25519::Public>("Bob"), ())],
    }),
```

* `cargo build` and then `cargo run -- --dev`

When the node starts, only the account ids added in the genesis config of this module would be able to send extrinsics to the runtime.

New account ids could be added to the whitelist by calling the module's `add_account` function using `root` key as origin.

## Sample

The usage of this pallet are demonstrated in the [Substrate permissioning sample](https://github.com/gautamdhameja/substrate-permissioning).

## Potential extension:

* The addition and removal of account id's to the whitelist could also be done using other governance methods instead of root.
* The logic can be reversed to maintain a blacklist of accounts which cannot send extrinsics to the runtime.

## Disclaimer

This code not audited and reviewed for production use cases. You can expect bugs and security vulnerabilities. Do not use it as-is in real applications.
