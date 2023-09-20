#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// SPDX-License-Identifier: MIT

#[openbrush::implementation(Ownable, AccessControl)]
#[openbrush::contract]
mod my_admin_access {
    use archisinal_lib::impls::admin_access::*;
    use archisinal_lib::impls::shared::consts::ADMIN;
    use archisinal_lib::traits::admin_access::*;
    use archisinal_lib::traits::events::admin_access::AdminAccessEvents;
    use archisinal_lib::traits::ProjectResult;
    use ink::codegen::{EmitEvent, Env};
    use openbrush::traits::Storage;

    #[ink(event)]
    pub struct AdminAdded {
        #[ink(topic)]
        caller: AccountId,
        /// The account id of the added admin.
        #[ink(topic)]
        account_id: AccountId,
    }

    #[ink(event)]
    pub struct AdminRemoved {
        #[ink(topic)]
        caller: AccountId,
        /// The account id of the removed admin.
        #[ink(topic)]
        account_id: AccountId,
    }

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access_control: access_control::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();

            ownable::Internal::_init_with_owner(&mut instance, caller);
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            access_control::AccessControl::grant_role(&mut instance, ADMIN, Some(caller))
                .expect("Failed to grant role");

            instance
        }
    }

    impl AdminAccessImpl for Contract {}

    impl AdminAccess for Contract {
        #[ink(message)]
        fn add_admin(&mut self, account_id: AccountId) -> ProjectResult<()> {
            AdminAccessImpl::add_admin(self, account_id)
        }

        #[ink(message)]
        fn remove_admin(&mut self, account_id: AccountId) -> ProjectResult<()> {
            AdminAccessImpl::remove_admin(self, account_id)
        }

        #[ink(message)]
        fn is_admin(&self, account_id: AccountId) -> bool {
            AdminAccessImpl::is_admin(self, account_id)
        }
    }

    impl AdminAccessEvents for Contract {
        fn emit_admin_added(
            &self,
            caller: openbrush::traits::AccountId,
            account_id: openbrush::traits::AccountId,
        ) {
            self.env().emit_event(AdminAdded { caller, account_id });
        }

        fn emit_admin_removed(
            &self,
            caller: openbrush::traits::AccountId,
            account_id: openbrush::traits::AccountId,
        ) {
            self.env().emit_event(AdminRemoved { caller, account_id });
        }
    }
}
