/// SPDX-License-Identifier: MIT
pub use data::*;
pub use impls::*;

pub mod data;
pub mod impls;

#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum AccountType {
    User,
    Creator,
}
