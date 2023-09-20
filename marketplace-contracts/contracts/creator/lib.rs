#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// SPDX-License-Identifier: MIT
pub use crate::creator::*;

/// This is the implementation of the creator contract.
///
/// The creator contract is responsible for creating collections,
/// and managing the creator metadata. Designed to be extensible in the future.
#[openbrush::implementation(Ownable, Upgradeable)]
#[openbrush::contract]
mod creator {
    use archisinal_lib::impls::creator::impls::{CreatorImpl, CreatorInternal};
    use archisinal_lib::impls::user::data::UserData;
    use archisinal_lib::impls::user::impls::UserImpl;
    use archisinal_lib::impls::{creator, user};
    use archisinal_lib::traits::creator::*;
    use archisinal_lib::traits::events::creator::CreatorEvents;
    use archisinal_lib::traits::events::user::UserEvents;
    use archisinal_lib::traits::user::*;
    use archisinal_lib::traits::ProjectResult;
    use ink::codegen::{EmitEvent, Env};
    use ink::env::DefaultEnvironment;
    use ink::{EnvAccess, ToAccountId};
    use openbrush::contracts::ownable;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(event)]
    pub struct CollectionCreated {
        /// The account id of the creator.
        #[ink(topic)]
        creator: AccountId,
        /// The account id of the collection created.
        #[ink(topic)]
        collection: AccountId,
        /// The index of the collection created.
        #[ink(topic)]
        index: u32,
    }

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
        #[storage_field]
        pub creator: creator::data::Data,
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

            UserImpl::set_user_data(&mut instance, data).unwrap();

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

    impl CreatorInternal for Contract {
        fn _instantiate_collection(
            &mut self,
            name: String,
            uri: String,
            royalty: u32,
            additional_info: String,
            code_hash: Hash,
        ) -> ProjectResult<openbrush::traits::AccountId> {
            let contract =
                arch_nft::ContractRef::new(royalty, Some(name), Some(uri), Some(additional_info))
                    .code_hash(code_hash)
                    .endowment(0)
                    .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                    .instantiate();

            Ok(contract.to_account_id())
        }
    }

    impl CreatorImpl for Contract {}

    impl Creator for Contract {
        #[ink(message)]
        fn create_collection(
            &mut self,
            name: String,
            uri: String,
            royalty: u32,
            additional_info: String,
            code_hash: Hash,
        ) -> ProjectResult<openbrush::traits::AccountId> {
            CreatorImpl::create_collection(self, name, uri, royalty, additional_info, code_hash)
        }

        #[ink(message)]
        fn get_collection_count(&self) -> u32 {
            CreatorImpl::get_collection_count(self)
        }

        #[ink(message)]
        fn get_collection_id_by_index(
            &self,
            index: u32,
        ) -> ProjectResult<openbrush::traits::AccountId> {
            CreatorImpl::get_collection_id_by_index(self, index)
        }
    }

    impl CreatorEvents for Contract {
        fn emit_create_collection(&self, creator: AccountId, collection: AccountId, index: u32) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<CollectionCreated>(
                self.env(),
                CollectionCreated {
                    creator,
                    collection,
                    index,
                },
            );
        }
    }

    impl UserEvents for Contract {
        fn emit_user_data_set(&self, user_data: UserData) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<Self>>::emit_event::<UserDataSet>(
                self.env(),
                UserDataSet { user_data },
            );
        }
    }
}
