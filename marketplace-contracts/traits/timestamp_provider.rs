#[openbrush::trait_definition]
pub trait TimestampProvider {
    #[ink(message)]
    fn timestamp(&self) -> u64;
}

#[openbrush::wrapper]
pub type TimestampRef = dyn TimestampProvider;
