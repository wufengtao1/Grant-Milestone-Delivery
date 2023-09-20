use crate::impls::account_manager::AccountType;
use ink::primitives::Hash;
use openbrush::storage::Mapping;
use openbrush::traits::AccountId;

/// The main data storage of the account manager.
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    /// The mapping of (user address, Creator/User) to account id of the contract.
    pub accounts: Mapping<(AccountId, AccountType), AccountId>,
    /// The code hash of the creator contract.
    #[lazy]
    pub creator_code_hash: Hash,
    /// The code hash of the user contract.
    #[lazy]
    pub user_code_hash: Hash,
}
