/// SPDX-License-Identifier: MIT
use crate::traits::ProjectResult;
use ink::primitives::{AccountId, Hash};

/// Account manager trait
///
/// This trait is used to manage accounts and creators
#[openbrush::trait_definition]
pub trait AccountManager {
    /// Create an account
    ///
    /// This function will deploy an account contract for the caller
    /// using the user code hash
    ///
    /// # Errors
    ///
    /// * `ArchisinalError::AccountAlreadyExists` - If the account already exists
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - Ok if the account was created, otherwise an error
    ///
    /// # Emits
    ///
    /// * `AccountCreated` - If the account was created successfully
    #[ink(message)]
    fn create_account(&mut self) -> ProjectResult<()>;

    /// Create a creator account
    ///
    /// This function will deploy a creator account contract for the caller
    /// using the creator code hash
    ///
    /// # Errors
    ///
    /// * `ArchisinalError::AccountAlreadyExists` - If the account already exists
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - Ok if the account was created, otherwise an error
    ///
    /// # Emits
    ///
    /// * `CreatorAccountCreated` - If the creator account was created successfully
    #[ink(message)]
    fn create_creator_account(&mut self) -> ProjectResult<()>;

    /// Get an account
    ///
    /// This function will get an account contract address
    ///
    /// # Arguments
    ///
    /// * `account_id` - An address of the user, for which to get the account contract address
    ///
    /// # Returns
    ///
    /// * `Option<AccountId>` - An account contract address if it exists, otherwise None
    #[ink(message)]
    fn get_account(&self, account_id: AccountId) -> Option<AccountId>;

    /// Get a creator account
    ///
    /// This function will get a creator account contract address
    ///
    /// # Arguments
    ///
    /// * `account_id` - An address of the user, for which to get the creator contract address
    ///
    /// # Returns
    ///
    /// * `Option<AccountId>` - A creator account contract address if it exists, otherwise None
    #[ink(message)]
    fn get_creator_account(&self, account_id: AccountId) -> Option<AccountId>;

    /// Get the creator code hash
    ///
    /// This function will get the creator code hash, which is used to deploy creator accounts
    ///
    /// # Returns
    ///
    /// * `Hash` - The creator code hash
    #[ink(message)]
    fn get_creator_code_hash(&self) -> Hash;

    /// Get the user code hash
    ///
    /// This function will get the user code hash, which is used to deploy user accounts
    ///
    /// # Returns
    ///
    /// * `Hash` - The user code hash
    #[ink(message)]
    fn get_user_code_hash(&self) -> Hash;

    /// Set the creator code hash
    ///
    /// This function will set the creator code hash, which is used to deploy creator accounts
    ///
    /// # Arguments
    ///
    /// * `code_hash` - The creator code hash
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - Ok if the code hash was set, otherwise an error
    ///
    /// # Emits
    ///
    /// * `CreatorCodeHashSet` - If the creator code hash was set successfully
    #[ink(message)]
    fn set_creator_code_hash(&mut self, code_hash: Hash) -> ProjectResult<()>;

    /// Set the user code hash
    ///
    /// This function will set the user code hash, which is used to deploy user accounts
    ///
    /// # Arguments
    ///
    /// * `code_hash` - The user code hash
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - Ok if the code hash was set, otherwise an error
    ///
    /// # Emits
    ///
    /// * `UserCodeHashSet` - If the user code hash was set successfully
    #[ink(message)]
    fn set_user_code_hash(&mut self, code_hash: Hash) -> ProjectResult<()>;
}
