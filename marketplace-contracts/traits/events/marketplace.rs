/// SPDX-License-Identifier: MIT
use crate::impls::shared::currency::Currency;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;
use openbrush::contracts::psp34::Id;

pub trait MarketplaceEvents {
    fn emit_list_nft(
        &self,
        listing_id: u128,
        creator: AccountId,
        collection: AccountId,
        token_id: Id,
        price: u128,
        currency: Currency,
    );

    fn emit_cancel_listing(&self, caller: AccountId, listing_id: u128);

    fn emit_buy_nft(&self, buyer: AccountId, listing_id: u128);

    fn emit_buy_batch(&self, buyer: AccountId, listing_ids: Vec<u128>);
}
