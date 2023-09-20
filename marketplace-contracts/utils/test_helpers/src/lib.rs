/// SPDX-License-Identifier: MIT

/// This macro is used to get the address of an account.
#[macro_export]
macro_rules! address_of {
    ($account:ident) => {
        ink::primitives::AccountId::from(ink_e2e::$account::<PolkadotConfig>().account_id().0)
    };
}
