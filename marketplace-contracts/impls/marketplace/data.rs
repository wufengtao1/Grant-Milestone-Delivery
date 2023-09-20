/// SPDX-License-Identifier: MIT
use openbrush::contracts::psp34::Id;
use openbrush::storage::Mapping;
use openbrush::traits::AccountId;

use crate::impls::shared::currency::Currency;

/// The main data storage of the marketplace.
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The total number of listings.
    #[lazy]
    pub listing_count: u128,
    /// The mapping of listing id to listing.
    pub listings: Mapping<u128, Listing>,
}

/// The listing data structure.
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Listing {
    /// The id of the listing.
    pub id: u128,
    /// The creator of the listing.
    pub creator: AccountId,
    /// The collection of the listing.
    pub collection: AccountId,
    /// The token id of the listing.
    pub token_id: Id,
    /// The price of the listing.
    pub price: u128,
    /// The currency of the listing.
    pub currency: Currency,
    /// The status of the listing.
    pub status: ListingStatus,
    /// Royalty of the listing, set automatically by deriving the value from the collection contract.
    pub royalty: u32,
}

/// The listing status enum.
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum ListingStatus {
    /// The listing is on sale.
    OnSale,
    /// The listing is sold.
    Sold,
    /// The listing is cancelled.
    Cancelled,
}
