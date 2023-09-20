#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// SPDX-License-Identifier: MIT
pub use crate::user::*;

/// # User contract
///
/// This contract is responsible for creating and managing user accounts.
/// It contains to save user social data. Will be extended in the future.
#[openbrush::implementation(Ownable, Upgradeable)]
#[openbrush::contract]
mod user {
    use archisinal_lib::impls::user;
    use archisinal_lib::impls::user::data::UserData;
    use archisinal_lib::impls::user::impls::UserImpl;
    use archisinal_lib::traits::events::user::UserEvents;
    use archisinal_lib::traits::user::*;
    use archisinal_lib::traits::ProjectResult;
    use ink::codegen::{EmitEvent, Env};
    use openbrush::contracts::ownable;
    use openbrush::traits::Storage;

    #[ink(event)]
    pub struct UserDataSet {
        /// New user_data.
        pub user_data: UserData,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pub ownable: ownable::Data,
        #[storage_field]
        pub user_data: user::data::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let mut instance = Self::default();

            ownable::Internal::_init_with_owner(&mut instance, owner);

            instance
        }

        #[ink(constructor, default)]
        pub fn default_constructor() -> Self {
            let mut instance = Self::default();

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());

            instance
        }

        #[ink(constructor)]
        pub fn new_with_data(owner: AccountId, data: UserData) -> Self {
            let mut instance = Self::default();

            ownable::Internal::_init_with_owner(&mut instance, owner);

            UserImpl::_set_user_data(&mut instance, data).unwrap();

            instance
        }
    }

    impl UserImpl for Contract {}

    impl User for Contract {
        #[ink(message)]
        fn get_user_data(&self) -> UserData {
            UserImpl::get_user_data(self)
        }

        #[ink(message)]
        fn set_user_data(&mut self, user_data: UserData) -> ProjectResult<()> {
            UserImpl::set_user_data(self, user_data)
        }
    }

    impl UserEvents for Contract {
        fn emit_user_data_set(&self, data: UserData) {
            self.env().emit_event(UserDataSet { user_data: data });
        }
    }
}
