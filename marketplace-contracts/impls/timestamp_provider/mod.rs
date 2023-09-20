use openbrush::traits::DefaultEnv;

pub trait TimestampProviderImpl: Sized + DefaultEnv {
    fn timestamp(&self) -> u64 {
        <Self as DefaultEnv>::env().block_timestamp()
    }
}
