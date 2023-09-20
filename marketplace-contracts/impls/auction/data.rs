/// SPDX-License-Identifier: MIT
use ink::primitives::AccountId;
use openbrush::contracts::psp34::Id;
use openbrush::storage::Mapping;

use crate::impls::shared::currency::Currency;

/// The main data storage of the auction.
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The total number of auctions.
    #[lazy]
    pub auction_count: u128,
    /// The mapping of auction id to auction.
    pub auctions: Mapping<u128, Auction>,
}

/// The auction data.
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Auction {
    /// The id of the auction.
    pub id: u128,
    /// The creator of the auction.
    pub creator: AccountId,
    /// The collection of the auction.
    pub collection: AccountId,
    /// The token id of the auction.
    pub token_id: Id,
    /// The start price of the auction.
    pub start_price: u128,
    /// The minimum bid step of the auction.
    pub min_bid_step: u128,
    /// The currency of the auction.
    pub currency: Currency,
    /// The start time of the auction.
    pub start_time: u64,
    /// The end time of the auction.
    pub end_time: u64,
    /// The current price of the auction.
    pub current_price: u128,
    /// The current bidder of the auction.
    pub current_bidder: Option<AccountId>,
    /// The status of the auction.
    pub status: AuctionStatus,
    /// Royalty of the auction, set automatically by deriving the value from the collection contract.
    pub royalty: u32,
}

/// The auction status.
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum AuctionStatus {
    /// The auction is waiting for auction.
    WaitingAuction,
    /// The auction is in auction.
    InAuction,
    /// The auction is waiting for claim.
    WaitingForClaim,
    /// The auction is ended.
    Ended,
    /// The auction is cancelled.
    Cancelled,
}
