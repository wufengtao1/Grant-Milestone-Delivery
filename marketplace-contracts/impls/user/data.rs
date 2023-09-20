/// SPDX-License-Identifier: MIT
use openbrush::contracts::psp34::Id;
use openbrush::traits::String;

/// Main data structure of the user contract.
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The nickname of the user.
    #[lazy]
    pub nick: Option<String>,
    /// The avatar of the user.
    #[lazy]
    pub avatar: Option<NFT>,
    /// The additional info of the user.
    #[lazy]
    pub addition_info: Option<String>,
}

/// The NFT data structure.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct NFT {
    /// The id of the NFT.
    pub id: Id,
    /// The uri of the NFT.
    pub uri: String,
    /// The contract address of the NFT.
    pub contract_address: String,
}

/// The user data structure, used for the `set_user_data` method.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct UserData {
    /// The nickname of the user.
    pub nick: Option<String>,
    /// The avatar of the user.
    pub avatar: Option<NFT>,
    /// The additional info of the user.
    pub addition_info: Option<String>,
}

impl From<&Data> for UserData {
    fn from(value: &Data) -> Self {
        Self {
            nick: value.nick.get_or_default(),
            avatar: value.avatar.get_or_default(),
            addition_info: value.addition_info.get_or_default(),
        }
    }
}
