/// SPDX-License-Identifier: MIT
use openbrush::contracts::ownable;
use openbrush::contracts::ownable::only_owner;
use openbrush::contracts::ownable::Ownable;
use openbrush::modifiers;
use openbrush::traits::{AccountId, Storage};
use openbrush::traits::{Hash, String};

use crate::impls::creator::data::Data;
use crate::traits::events::creator::CreatorEvents;
use crate::traits::{ArchisinalError, ProjectResult};

/// The creator internal implementation.
pub trait CreatorInternal {
    /// Instantiate a collection.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the collection.
    /// - `uri`: The URI of the collection.
    /// - `royalty`: The royalty of the collection.
    /// - `additional_info`: The additional info of the collection.
    /// - `code_hash`: The code hash of the collection.
    ///
    /// # Returns
    ///
    /// The collection address.
    ///
    /// # Note
    ///
    /// This function is implemented in the contract itself.
    fn _instantiate_collection(
        &mut self,
        name: String,
        uri: String,
        royalty: u32,
        additional_info: String,
        code_hash: Hash,
    ) -> ProjectResult<AccountId>;
}

/// The creator implementation.
///
/// # Note
///
/// See `crate::traits::Creator` for more information.
pub trait CreatorImpl:
    Storage<Data> + Ownable + Storage<ownable::Data> + CreatorInternal + CreatorEvents
{
    #[modifiers(only_owner)]
    fn create_collection(
        &mut self,
        name: String,
        uri: String,
        royalty: u32,
        additional_info: String,
        code_hash: Hash,
    ) -> ProjectResult<AccountId> {
        let collection_address =
            self._instantiate_collection(name, uri, royalty, additional_info, code_hash)?;

        let caller: AccountId = self.owner().ok_or(ArchisinalError::NoOwner)?;

        let count = self.get_collection_count();

        self.data::<Data>()
            .collection_addresses
            .insert(&count, &collection_address);

        self.data::<Data>().collection_count.set(
            &count
                .checked_add(1)
                .ok_or(ArchisinalError::IntegerOverflow)?,
        );

        self.emit_create_collection(caller, collection_address, count);

        Ok(collection_address)
    }

    fn get_collection_count(&self) -> u32 {
        self.data::<Data>().collection_count.get_or_default()
    }

    fn get_collection_id_by_index(&self, index: u32) -> ProjectResult<AccountId> {
        let collection_count = self.get_collection_count();
        if index >= collection_count {
            return Err(ArchisinalError::CollectionNotFound);
        }

        let collection_address = self.data::<Data>().collection_addresses.get(&index);

        if collection_address.is_none() {
            return Err(ArchisinalError::CollectionNotFound);
        }

        let collection_address = collection_address.unwrap();

        Ok(collection_address)
    }
}
