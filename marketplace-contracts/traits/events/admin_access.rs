/// SPDX-License-Identifier: MIT
use openbrush::traits::AccountId;

pub trait AdminAccessEvents {
    fn emit_admin_added(&self, caller: AccountId, account_id: AccountId);

    fn emit_admin_removed(&self, caller: AccountId, account_id: AccountId);
}
