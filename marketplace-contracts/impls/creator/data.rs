/// SPDX-License-Identifier: MIT
use openbrush::storage::Mapping;
use openbrush::traits::AccountId;

/// The main data storage of the marketplace.
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The total number of collections.
    #[lazy]
    pub collection_count: u32,
    /// The mapping of collection id to collection.
    pub collection_addresses: Mapping<u32, AccountId>,
}
