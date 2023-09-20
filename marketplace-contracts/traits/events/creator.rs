/// SPDX-License-Identifier: MIT
use ink::primitives::AccountId;

pub trait CreatorEvents {
    fn emit_create_collection(&self, creator: AccountId, collection: AccountId, index: u32);
}
