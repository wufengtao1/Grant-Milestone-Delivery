#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// SPDX-License-Identifier: MIT

/// # AccountManager contract
///
/// This contract is responsible for creating and managing user and creator accounts.
#[openbrush::implementation(Ownable, AccessControl, Upgradeable)]
#[openbrush::contract]
mod account_manager {
    use archisinal_lib::impls::account_manager;
    use archisinal_lib::impls::account_manager::AccountManagerImpl;
    use archisinal_lib::impls::account_manager::AccountType;
    use archisinal_lib::impls::admin_access::AdminAccessImpl;
    use archisinal_lib::impls::shared::consts::ADMIN;
    use archisinal_lib::traits::account_manager::*;
    use archisinal_lib::traits::admin_access::*;
    use archisinal_lib::traits::events::account_manager::AccountManagerEvents;
    use archisinal_lib::traits::events::admin_access::AdminAccessEvents;
    use archisinal_lib::traits::ProjectResult;
    use ink::codegen::{EmitEvent, Env, StaticEnv};
    use ink::env::DefaultEnvironment;
    use ink::{EnvAccess, ToAccountId};
    use openbrush::traits::Storage;

    #[ink(event)]
    pub struct AccountCreated {
        /// The account id of the contract owner.
        #[ink(topic)]
        pub account_id: AccountId,
        /// The account id of the deployed user contract.
        #[ink(topic)]
        pub contract_id: AccountId,
    }

    #[ink(event)]
    pub struct CreatorAccountCreated {
        /// The account id of the contract owner.
        #[ink(topic)]
        pub account_id: AccountId,
        /// The account id of the deployed creator contract.
        #[ink(topic)]
        pub contract_id: AccountId,
    }

    #[ink(event)]
    pub struct CodeHashSet {
        /// The code hash of the user contract.
        #[ink(topic)]
        pub code_hash: Hash,
        #[ink(topic)]
        pub account_type: AccountType,
    }

    #[ink(event)]
    pub struct AdminAdded {
        /// The account id of the caller.
        #[ink(topic)]
        pub caller: AccountId,
        /// The account id of the added admin.
        #[ink(topic)]
        pub account_id: AccountId,
    }

    #[ink(event)]
    pub struct AdminRemoved {
        /// The account id of the caller.
        #[ink(topic)]
        pub caller: AccountId,
        /// The account id of the removed admin.
        #[ink(topic)]
        pub account_id: AccountId,
    }

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        account_manager: account_manager::Data,
        #[storage_field]
        access_control: access_control::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(user_code_hash: Hash, creator_code_hash: Hash) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();

            ownable::Internal::_init_with_owner(&mut instance, caller);
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            access_control::AccessControl::grant_role(&mut instance, ADMIN, Some(caller))
                .expect("Failed to grant role");

            AccountManagerImpl::set_user_code_hash(&mut instance, user_code_hash).unwrap();
            AccountManagerImpl::set_creator_code_hash(&mut instance, creator_code_hash).unwrap();

            instance
        }
    }

    impl AccountManagerImpl for Contract {
        fn create_account(&mut self) -> ProjectResult<()> {
            let caller = self.env().caller();

            let contract = user::ContractRef::new(caller)
                .code_hash(AccountManagerImpl::get_user_code_hash(self))
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            self._add_account(caller, contract.to_account_id())?;

            Ok(())
        }

        fn create_creator_account(&mut self) -> ProjectResult<()> {
            let caller = self.env().caller();

            let contract = creator::ContractRef::new(caller)
                .code_hash(AccountManagerImpl::get_creator_code_hash(self))
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            self._add_creator(caller, contract.to_account_id())?;

            Ok(())
        }
    }

    impl AdminAccessImpl for Contract {}

    impl AccountManager for Contract {
        #[ink(message)]
        fn create_account(&mut self) -> ProjectResult<()> {
            AccountManagerImpl::create_account(self)
        }

        #[ink(message)]
        fn create_creator_account(&mut self) -> ProjectResult<()> {
            AccountManagerImpl::create_creator_account(self)
        }

        #[ink(message)]
        fn get_account(&self, account_id: AccountId) -> Option<AccountId> {
            AccountManagerImpl::get_account(self, account_id)
        }

        #[ink(message)]
        fn get_creator_account(&self, account_id: AccountId) -> Option<AccountId> {
            AccountManagerImpl::get_creator_account(self, account_id)
        }

        #[ink(message)]
        fn get_creator_code_hash(&self) -> Hash {
            AccountManagerImpl::get_creator_code_hash(self)
        }

        #[ink(message)]
        fn get_user_code_hash(&self) -> Hash {
            AccountManagerImpl::get_user_code_hash(self)
        }

        #[ink(message)]
        fn set_creator_code_hash(&mut self, code_hash: Hash) -> ProjectResult<()> {
            AccountManagerImpl::set_creator_code_hash(self, code_hash)
        }

        #[ink(message)]
        fn set_user_code_hash(&mut self, code_hash: Hash) -> ProjectResult<()> {
            AccountManagerImpl::set_user_code_hash(self, code_hash)
        }
    }

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

    impl AccountManagerEvents for Contract {
        fn emit_account_created(&self, account_id: AccountId, contract_id: AccountId) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<AccountCreated>(
                Self::env(),
                AccountCreated {
                    account_id,
                    contract_id,
                },
            );
        }

        fn emit_creator_created(&self, account_id: AccountId, contract_id: AccountId) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<
                CreatorAccountCreated,
            >(
                Self::env(),
                CreatorAccountCreated {
                    account_id,
                    contract_id,
                },
            );
        }

        fn emit_code_hash_set(&self, code_hash: Hash, account_type: AccountType) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<CodeHashSet>(
                Self::env(),
                CodeHashSet {
                    code_hash,
                    account_type,
                },
            );
        }
    }

    impl AdminAccessEvents for Contract {
        fn emit_admin_added(&self, caller: AccountId, account_id: AccountId) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<AdminAdded>(
                Self::env(),
                AdminAdded { caller, account_id },
            );
        }

        fn emit_admin_removed(&self, caller: AccountId, account_id: AccountId) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<AdminRemoved>(
                Self::env(),
                AdminRemoved { caller, account_id },
            );
        }
    }
}
