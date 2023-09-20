#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// SPDX-License-Identifier: MIT

#[openbrush::implementation(PSP22, PSP22Mintable, PSP22Burnable)]
#[openbrush::contract]
mod my_psp22 {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct MyPsp22 {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl MyPsp22 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
