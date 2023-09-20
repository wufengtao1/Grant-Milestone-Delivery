/// SPDX-License-Identifier: MIT
use openbrush::traits::String;

/// The collection data.
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The name of the collection.
    #[lazy]
    pub name: Option<String>,
    /// The symbol of the collection.
    #[lazy]
    pub uri: Option<String>,
    /// The additional info of the collection.
    #[lazy]
    pub additional_info: Option<String>,
    /// The royalty of the collection.
    #[lazy]
    pub royalty: u32,
}
