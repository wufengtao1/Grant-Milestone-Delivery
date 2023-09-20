/// SPDX-License-Identifier: MIT
use crate::impls::shared::currency::Currency;
use ink::primitives::AccountId;
use openbrush::contracts::psp34::Id;

pub trait AuctionEvents {
    #[allow(clippy::too_many_arguments)]
    fn emit_auction_created(
        &self,
        auction_id: u128,
        creator: AccountId,
        collection: AccountId,
        token_id: Id,
        start_price: u128,
        min_bid_step: u128,
        currency: Currency,
        start_time: u64,
        end_time: u64,
    );

    fn emit_auction_started(&self, caller: AccountId, auction_id: u128);

    fn emit_auction_ended(&self, caller: AccountId, auction_id: u128);

    fn emit_no_bids(&self, caller: AccountId, auction_id: u128);

    fn emit_auction_cancelled(&self, caller: AccountId, auction_id: u128);

    fn emit_bid_placed(&self, auction_id: u128, bidder: AccountId, bid: u128);

    fn emit_nft_claimed(&self, caller: AccountId, auction_id: u128);
}
