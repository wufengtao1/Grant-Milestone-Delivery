/// SPDX-License-Identifier: MIT
use crate::traits::ProjectResult;
use ink::primitives::{AccountId, Hash};
use openbrush::contracts::ownable::*;
use openbrush::traits::String;

/// Creator trait definition
///
/// # Note
///
/// This trait is used by creators to create collections,
/// also this trait should be used with `User` trait.
#[openbrush::trait_definition]
pub trait Creator {
    /// Creates a new collection.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the collection.
    /// * `uri` - The uri of the collection.
    /// * `royalty` - The royalty of the collection.
    /// * `additional_info` - The additional info of the collection.
    /// * `code_hash` - The code hash of the collection contract.
    ///
    /// # Returns
    ///
    /// * `AccountId` - The id of the collection, if it was created successfully, otherwise an error.
    ///
    /// # Errors
    ///
    /// * Returns `ProjectError::OwnableError` if the caller is not the owner.
    /// * Returns `ProjectError::CollectionAlreadyExists` if the collection already exists.
    ///
    /// # Emits
    ///
    /// * `CollectionCreated` - If the collection was created successfully.
    #[ink(message)]
    fn create_collection(
        &mut self,
        name: String,
        uri: String,
        royalty: u32,
        additional_info: String,
        code_hash: Hash,
    ) -> ProjectResult<AccountId>;

    /// Get collection count
    ///
    /// # Note
    /// The index of the collection is from `0` to `collection_count - 1` inclusively.
    ///
    /// # Returns
    ///
    /// * `u32` - The amount of collections.
    #[ink(message)]
    fn get_collection_count(&self) -> u32;

    /// Get collection id by index
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the collection.
    ///
    /// # Returns
    ///
    /// * `ProjectResult<AccountId>` - The id of the collection, if it was created successfully, otherwise an error.
    ///
    /// # Errors
    ///
    /// * Returns `ProjectError::CollectionNotFound` if the collection not found.
    #[ink(message)]
    fn get_collection_id_by_index(&self, index: u32) -> ProjectResult<AccountId>;
}

#[openbrush::wrapper]
pub type CreatorRef = dyn Creator + Ownable;
