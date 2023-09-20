/// SPDX-License-Identifier: MIT
use crate::impls::user::data::UserData;

pub trait UserEvents {
    fn emit_user_data_set(&self, data: UserData);
}
