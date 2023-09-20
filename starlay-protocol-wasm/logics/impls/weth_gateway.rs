// Copyright 2023 Asynmatrix Pte. Ltd.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use crate::traits::weth_gateway::*;
use crate::traits::{
    pool::PoolRef,
    weth::*,
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        psp22::*,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
    },
};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);
#[derive(Debug, Default)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// Account Id of Wrapped Native Token(PSP22)
    pub weth: Option<AccountId>,
}

pub trait Internal {
    fn _initialize(&mut self, weth: AccountId);
    fn _safe_transfer_eth(&self, to: AccountId, value: Balance) -> Result<()>;
    fn _emit_deposit_eth_event_(&self, pool: AccountId, from: AccountId, value: Balance);
    fn _emit_withdraw_eth_event_(&self, pool: AccountId, to: AccountId, value: Balance);
    fn _emit_borrow_eth_event_(&self, pool: AccountId, to: AccountId, value: Balance);
    fn _emit_repay_eth_event_(&self, pool: AccountId, from: AccountId, value: Balance);
    fn _weth_address(&self) -> Option<AccountId>;
}

impl<T: Storage<Data> + Storage<ownable::Data>> Internal for T {
    default fn _initialize(&mut self, weth: AccountId) {
        self.data::<Data>().weth = Some(weth);
    }

    default fn _safe_transfer_eth(&self, to: AccountId, value: Balance) -> Result<()> {
        let transfer_result = Self::env().transfer(to, value);
        if transfer_result.is_err() {
            return Err(Error::SafeETHTransferFailed)
        }
        Ok(())
    }

    default fn _weth_address(&self) -> Option<AccountId> {
        self.data::<Data>().weth
    }

    default fn _emit_deposit_eth_event_(
        &self,
        _pool: AccountId,
        _from: AccountId,
        _value: Balance,
    ) {
    }

    default fn _emit_withdraw_eth_event_(&self, _pool: AccountId, _to: AccountId, _value: Balance) {
    }

    default fn _emit_borrow_eth_event_(&self, _pool: AccountId, _to: AccountId, _value: Balance) {}

    default fn _emit_repay_eth_event_(&self, _pool: AccountId, _from: AccountId, _value: Balance) {}
}

impl<T> WETHGateway for T
where
    T: Storage<Data> + Storage<ownable::Data>,
{
    default fn deposit_eth(&mut self, pool: AccountId) -> Result<()> {
        let deposit_value = Self::env().transferred_value();
        let caller = Self::env().caller();
        let weth = self._weth_address();
        if weth.is_none() {
            return Err(Error::WethIsNotSet)
        }
        let _weth = weth.unwrap();

        WETHRef::deposit_builder(&_weth)
            .transferred_value(deposit_value)
            .invoke()?;
        WETHRef::approve(&_weth, pool, deposit_value)?;
        PoolRef::mint_to(&pool, caller, deposit_value)?;
        self._emit_deposit_eth_event_(pool, caller, deposit_value);
        Ok(())
    }

    default fn withdraw_eth(&mut self, pool: AccountId, amount: Balance) -> Result<()> {
        let caller = Self::env().caller();
        let contract_address = Self::env().account_id();
        let user_balance: Balance = PoolRef::balance_of(&pool, caller);
        let mut amount_to_withdraw: Balance = amount;

        if amount == u128::MAX {
            amount_to_withdraw = user_balance;
        }

        let weth = self._weth_address();
        if weth.is_none() {
            return Err(Error::WethIsNotSet)
        }
        let _weth = weth.unwrap();

        PoolRef::transfer_from(
            &pool,
            caller,
            contract_address,
            amount_to_withdraw,
            Vec::<u8>::new(),
        )?;
        PoolRef::redeem_underlying(&pool, amount_to_withdraw)?;
        WETHRef::withdraw(&_weth, amount_to_withdraw)?;
        self._emit_withdraw_eth_event_(pool, caller, amount_to_withdraw);
        self._safe_transfer_eth(caller, amount_to_withdraw)
    }

    default fn repay_eth(&mut self, pool: AccountId, amount: Balance) -> Result<()> {
        let transferred_value = Self::env().transferred_value();
        let caller = Self::env().caller();
        let mut payback_amount = PoolRef::borrow_balance_current(&pool, caller)?;
        if amount < payback_amount {
            payback_amount = amount;
        }
        if transferred_value < payback_amount {
            return Err(Error::InsufficientPayback)
        }

        let weth = self._weth_address();
        if weth.is_none() {
            return Err(Error::WethIsNotSet)
        }
        let _weth = weth.unwrap();

        WETHRef::deposit_builder(&_weth)
            .transferred_value(payback_amount)
            .invoke()?;
        WETHRef::approve(&_weth, pool, payback_amount)?;
        PoolRef::repay_borrow_behalf(&pool, caller, payback_amount)?;
        self._emit_repay_eth_event_(pool, caller, payback_amount);
        if transferred_value > payback_amount {
            self._safe_transfer_eth(caller, transferred_value - payback_amount)?;
        }
        Ok(())
    }

    default fn borrow_eth(&mut self, pool: AccountId, amount: Balance) -> Result<()> {
        let caller = Self::env().caller();
        let weth = self._weth_address();
        if weth.is_none() {
            return Err(Error::WethIsNotSet)
        }
        let _weth = weth.unwrap();

        PoolRef::borrow_for(&pool, caller, amount)?;
        WETHRef::withdraw(&_weth, amount)?;
        self._emit_borrow_eth_event_(pool, caller, amount);
        self._safe_transfer_eth(caller, amount)
    }

    default fn emergency_token_transfer(
        &mut self,
        token: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<()> {
        PSP22Ref::transfer(&token, to, amount, Vec::<u8>::new())?;
        Ok(())
    }

    default fn emergency_ether_transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        self._safe_transfer_eth(to, amount)
    }

    default fn get_weth_address(&self) -> Option<AccountId> {
        self._weth_address()
    }
}
