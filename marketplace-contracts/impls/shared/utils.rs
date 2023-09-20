/// SPDX-License-Identifier: MIT
use crate::traits::{ArchisinalError, ProjectResult};

/// Apply the fee to the price
///
/// # Arguments
///
/// * `price` - The price to apply the fee to
///
/// # Returns
///
/// * `ProjectResult<u128>` - The price with the fee applied, if successful otherwise an error
///
/// # Errors
///
/// * `ArchisinalError::IntegerOverflow` - If an integer overflow occurs
///
/// # Note
///
/// The fee is calculated as `price * royalty / 10000`
pub fn apply_fee(price: &u128, royalty: u32) -> ProjectResult<u128> {
    let fee = price
        .checked_mul(royalty as u128)
        .ok_or(ArchisinalError::IntegerOverflow)?
        .checked_div(10000)
        .unwrap();

    price
        .checked_add(fee)
        .ok_or(ArchisinalError::IntegerOverflow)
}
