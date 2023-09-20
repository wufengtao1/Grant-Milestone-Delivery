/// SPDX-License-Identifier: MIT
use crate::traits::ProjectResult;
use ink::primitives::AccountId;

/// AdminAccess trait definition
///
/// # Note
///
/// This trait is used to manage admins of the contract
#[openbrush::trait_definition]
pub trait AdminAccess {
    /// Add an admin
    ///
    /// # Note
    ///
    /// This function can only be called by an admin
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account id of the admin to add
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - Ok if the admin was added, otherwise an error
    ///
    /// # Emits
    ///
    /// * `AdminAdded` - If the admin was added successfully
    #[ink(message)]
    fn add_admin(&mut self, account_id: AccountId) -> ProjectResult<()>;

    /// Remove an admin
    ///
    /// # Note
    ///
    /// This function can only be called by an admin
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account id of the admin to remove
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - Ok if the admin was removed, otherwise an error
    ///
    /// # Emits
    ///
    /// * `AdminRemoved` - If the admin was removed successfully
    #[ink(message)]
    fn remove_admin(&mut self, account_id: AccountId) -> ProjectResult<()>;

    /// Check if an account is an admin
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account id of the admin to check
    ///
    /// # Returns
    ///
    /// * `bool` - True if the account is an admin, otherwise false
    #[ink(message)]
    fn is_admin(&self, account_id: AccountId) -> bool;
}
