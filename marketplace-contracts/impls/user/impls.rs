/// SPDX-License-Identifier: MIT
use openbrush::contracts::ownable;
use openbrush::contracts::ownable::only_owner;
use openbrush::contracts::ownable::Ownable;
use openbrush::modifiers;
use openbrush::traits::Storage;

use crate::impls::user::data::{Data, UserData};
use crate::traits::events::user::UserEvents;
use crate::traits::ProjectResult;

/// The user implementation.
///
/// # Note
///
/// See `crate::traits::User` for more information.
pub trait UserImpl: Storage<Data> + Ownable + Storage<ownable::Data> + UserEvents {
    fn get_user_data(&self) -> UserData {
        UserData::from(self.data::<Data>())
    }

    #[modifiers(only_owner)]
    fn set_user_data(&mut self, user_data: UserData) -> ProjectResult<()> {
        self._set_user_data(user_data)
    }

    fn _set_user_data(&mut self, user_data: UserData) -> ProjectResult<()> {
        self.data::<Data>().nick.set(&user_data.nick);
        self.data::<Data>().avatar.set(&user_data.avatar);
        self.data::<Data>()
            .addition_info
            .set(&user_data.addition_info);

        self.emit_user_data_set(user_data);

        Ok(())
    }
}
