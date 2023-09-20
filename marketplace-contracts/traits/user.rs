/// SPDX-License-Identifier: MIT
use crate::impls::user::data::UserData;
use crate::traits::ProjectResult;
use openbrush::contracts::ownable::*;

/// User trait definition
///
/// # Note
///
/// This trait is used to manage user social data
#[openbrush::trait_definition]
pub trait User {
    /// Get the UserData of the user.
    ///
    /// # Returns
    ///
    /// * `UserData` - The UserData of the user.
    #[ink(message)]
    fn get_user_data(&self) -> UserData;

    /// Set the UserData of the user.
    ///
    /// # Arguments
    ///
    /// * `user_info` - The UserData of the user.
    ///
    /// # Returns
    ///
    /// * `ProjectResult<()>` - The result of the operation, if it was created successfully, otherwise an error.
    ///
    /// # Emits
    ///
    /// * `UserDataSet` - If the user data was changed successfully.
    #[ink(message)]
    fn set_user_data(&mut self, user_info: UserData) -> ProjectResult<()>;
}

#[openbrush::wrapper]
pub type UserRef = dyn User + Ownable;
