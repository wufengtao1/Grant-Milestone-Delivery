/// SPDX-License-Identifier: MIT
use ink::primitives::{AccountId, Hash};
use openbrush::contracts::ownable::Ownable;
use openbrush::traits::Storage;

use crate::impls::account_manager::{AccountType, Data};
use crate::impls::admin_access::AdminAccessImpl;
use crate::traits::events::account_manager::AccountManagerEvents;
use crate::traits::{ArchisinalError, ProjectResult};

/// The account manager implementation.
///
/// # Note
///
/// See `crate::traits::AccountManager` for more information.
pub trait AccountManagerImpl:
    Storage<Data> + Ownable + AdminAccessImpl + AccountManagerEvents
{
    fn create_account(&mut self) -> ProjectResult<()>;

    fn create_creator_account(&mut self) -> ProjectResult<()>;

    fn get_account(&self, account_id: AccountId) -> Option<AccountId> {
        self.data::<Data>()
            .accounts
            .get(&(account_id, AccountType::User))
    }

    fn get_creator_account(&self, account_id: AccountId) -> Option<AccountId> {
        self.data::<Data>()
            .accounts
            .get(&(account_id, AccountType::Creator))
    }

    fn get_creator_code_hash(&self) -> Hash {
        self.data::<Data>().creator_code_hash.get_or_default()
    }

    fn get_user_code_hash(&self) -> Hash {
        self.data::<Data>().user_code_hash.get_or_default()
    }

    fn set_creator_code_hash(&mut self, code_hash: Hash) -> ProjectResult<()> {
        self._admin_or_owner()?;

        self.data::<Data>().creator_code_hash.set(&code_hash);

        self.emit_code_hash_set(code_hash, AccountType::Creator);

        Ok(())
    }

    fn set_user_code_hash(&mut self, code_hash: Hash) -> ProjectResult<()> {
        self._admin_or_owner()?;

        self.data::<Data>().user_code_hash.set(&code_hash);

        self.emit_code_hash_set(code_hash, AccountType::User);

        Ok(())
    }

    fn _add_account(&mut self, account_id: AccountId, contract: AccountId) -> ProjectResult<()> {
        if self.get_account(account_id).is_some() {
            return Err(ArchisinalError::AccountAlreadyExists);
        }

        self.data::<Data>()
            .accounts
            .insert(&(account_id, AccountType::User), &contract);

        self.emit_account_created(account_id, contract);

        Ok(())
    }

    fn _add_creator(&mut self, account_id: AccountId, contract: AccountId) -> ProjectResult<()> {
        if self.get_creator_account(account_id).is_some() {
            return Err(ArchisinalError::AccountAlreadyExists);
        }

        self.data::<Data>()
            .accounts
            .insert(&(account_id, AccountType::Creator), &contract);

        self.emit_creator_created(account_id, contract);

        Ok(())
    }
}
