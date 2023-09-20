use super::exp_no_err::Exp;
pub use crate::traits::{
    controller::*,
    pool::PoolRef,
};
use crate::{
    impls::price_oracle::PRICE_PRECISION,
    traits::{
        price_oracle::PriceOracleRef,
        types::WrappedU256,
    },
};
use core::ops::{
    Add,
    Div,
    Mul,
    Sub,
};
use ink::prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
    },
};
use primitive_types::U256;

mod utils;
use self::utils::{
    calculate_health_factor_from_balances,
    collateral_factor_max_mantissa,
    get_hypothetical_account_liquidity,
    liquidate_calculate_seize_tokens,
    BalanceDecreaseAllowedParam,
    GetHypotheticalAccountLiquidityInput,
    HypotheticalAccountLiquidityCalculationParam,
    LiquidateCalculateSeizeTokensInput,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// AccountId of managed Pools
    pub markets: Vec<AccountId>,
    /// Pair of pool and underlying
    pub markets_pair: Mapping<AccountId, AccountId>,
    /// Mapping of Pool and Collateral Factors
    pub collateral_factor_mantissa: Mapping<AccountId, WrappedU256>,
    /// Whether Pool has paused `Mint` Action
    pub mint_guardian_paused: Mapping<AccountId, bool>,
    /// Whether Pool has paused `Borrow` Action
    pub borrow_guardian_paused: Mapping<AccountId, bool>,
    /// Whether Pool has paused `Seize` Action
    pub seize_guardian_paused: bool,
    /// Whether Pool has paused `Transfer` Action
    pub transfer_guardian_paused: bool,
    /// Oracle's AccountId associated with this contract
    pub oracle: Option<AccountId>,
    /// Close Factor
    pub close_factor_mantissa: WrappedU256,
    /// Liquidation Incentive
    pub liquidation_incentive_mantissa: WrappedU256,
    /// Maximum that can be borrowed per Pool
    pub borrow_caps: Mapping<AccountId, Balance>,
    /// Manager's AccountId associated with this contract
    pub manager: Option<AccountId>,
    /// Flashloan Gateway's AccountId associated with this contract
    pub flashloan_gateway: Option<AccountId>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            markets: Default::default(),
            markets_pair: Default::default(),
            collateral_factor_mantissa: Default::default(),
            mint_guardian_paused: Default::default(),
            borrow_guardian_paused: Default::default(),
            seize_guardian_paused: Default::default(),
            transfer_guardian_paused: Default::default(),
            oracle: None,
            close_factor_mantissa: WrappedU256::from(U256::zero()),
            liquidation_incentive_mantissa: WrappedU256::from(U256::zero()),
            borrow_caps: Default::default(),
            manager: None,
            flashloan_gateway: None,
        }
    }
}

impl Default for PoolAttributes {
    fn default() -> Self {
        PoolAttributes {
            underlying: None,
            decimals: Default::default(),
            account_balance: Default::default(),
            account_borrow_balance: Default::default(),
            exchange_rate: Default::default(),
            total_borrows: Default::default(),
        }
    }
}

impl Default for PoolAttributesForWithdrawValidation {
    fn default() -> Self {
        PoolAttributesForWithdrawValidation {
            pool: None,
            underlying: None,
            liquidation_threshold: Default::default(),
            account_balance: Default::default(),
            account_borrow_balance: Default::default(),
        }
    }
}

pub trait Internal {
    fn _mint_allowed(&self, pool: AccountId, minter: AccountId, mint_amount: Balance)
        -> Result<()>;
    fn _mint_verify(
        &self,
        pool: AccountId,
        minter: AccountId,
        mint_amount: Balance,
        mint_tokens: Balance,
    ) -> Result<()>;
    fn _redeem_allowed(
        &self,
        pool: AccountId,
        redeemer: AccountId,
        amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()>;
    fn _redeem_verify(
        &self,
        pool: AccountId,
        redeemer: AccountId,
        redeem_amount: Balance,
    ) -> Result<()>;
    fn _borrow_allowed(
        &self,
        pool: AccountId,
        borrower: AccountId,
        borrow_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()>;
    fn _borrow_verify(
        &self,
        pool: AccountId,
        borrower: AccountId,
        borrow_amount: Balance,
    ) -> Result<()>;
    fn _repay_borrow_allowed(
        &self,
        pool: AccountId,
        payer: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
    ) -> Result<()>;
    fn _repay_borrow_verify(
        &self,
        pool: AccountId,
        payer: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        borrower_index: u128,
    ) -> Result<()>;
    fn _liquidate_borrow_allowed(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()>;
    fn _liquidate_borrow_verify(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        seize_tokens: Balance,
    ) -> Result<()>;
    fn _seize_allowed(
        &self,
        pool_collateral: AccountId,
        pool_borrowed: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        seize_tokens: Balance,
    ) -> Result<()>;
    fn _seize_verify(
        &self,
        pool_collateral: AccountId,
        pool_borrowed: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        seize_tokens: Balance,
    ) -> Result<()>;
    fn _transfer_allowed(
        &self,
        pool: AccountId,
        src: AccountId,
        dst: AccountId,
        transfer_tokens: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()>;
    fn _transfer_verify(
        &self,
        pool: AccountId,
        src: AccountId,
        dst: AccountId,
        transfer_tokens: Balance,
    ) -> Result<()>;
    fn _liquidate_calculate_seize_tokens(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        exchange_rate_mantissa: WrappedU256,
        repay_amount: Balance,
        pool_borrowed_attributes: Option<PoolAttributesForSeizeCalculation>,
        pool_collateral_attributes: Option<PoolAttributesForSeizeCalculation>,
    ) -> Result<Balance>;
    fn _assert_manager(&self) -> Result<()>;

    // admin functions
    fn _set_price_oracle(&mut self, new_oracle: AccountId) -> Result<()>;
    fn _support_market(
        &mut self,
        pool: &AccountId,
        underlying: &AccountId,
        collateral_factor_mantissa: Option<WrappedU256>,
    ) -> Result<()>;
    fn _set_flashloan_gateway(&mut self, flashloan_gateway: AccountId) -> Result<()>;
    fn _set_collateral_factor_mantissa(
        &mut self,
        pool: &AccountId,
        new_collateral_factor_mantissa: WrappedU256,
    ) -> Result<()>;
    fn _set_mint_guardian_paused(&mut self, pool: &AccountId, paused: bool) -> Result<()>;
    fn _set_borrow_guardian_paused(&mut self, pool: &AccountId, paused: bool) -> Result<()>;
    fn _set_seize_guardian_paused(&mut self, paused: bool) -> Result<()>;
    fn _set_transfer_guardian_paused(&mut self, paused: bool) -> Result<()>;
    fn _set_close_factor_mantissa(&mut self, new_close_factor_mantissa: WrappedU256) -> Result<()>;
    fn _set_liquidation_incentive_mantissa(
        &mut self,
        new_liquidation_incentive_mantissa: WrappedU256,
    ) -> Result<()>;
    fn _set_borrow_cap(&mut self, pool: &AccountId, new_cap: Balance) -> Result<()>;

    // view function
    fn _markets(&self) -> Vec<AccountId>;
    fn _market_of_underlying(&self, underlying: AccountId) -> Option<AccountId>;
    fn _flashloan_gateway(&self) -> Option<AccountId>;
    fn _collateral_factor_mantissa(&self, pool: AccountId) -> Option<WrappedU256>;
    fn _is_listed(&self, pool: AccountId) -> bool;
    fn _mint_guardian_paused(&self, pool: AccountId) -> Option<bool>;
    fn _borrow_guardian_paused(&self, pool: AccountId) -> Option<bool>;
    fn _seize_guardian_paused(&self) -> bool;
    fn _transfer_guardian_paused(&self) -> bool;
    fn _oracle(&self) -> Option<AccountId>;
    fn _close_factor_mantissa(&self) -> WrappedU256;
    fn _liquidation_incentive_mantissa(&self) -> WrappedU256;
    fn _borrow_cap(&self, pool: AccountId) -> Option<Balance>;
    fn _manager(&self) -> Option<AccountId>;

    fn _account_assets(
        &self,
        account: AccountId,
        token_modify: Option<AccountId>,
    ) -> Vec<AccountId>;
    fn _get_account_liquidity(&self, account: AccountId) -> Result<(U256, U256)>;
    fn _get_hypothetical_account_liquidity(
        &self,
        account: AccountId,
        token: Option<AccountId>,
        redeem_tokens: Balance,
        borrow_amount: Balance,
        caller_pool: Option<(AccountId, PoolAttributes)>,
    ) -> Result<(U256, U256)>;
    fn _calculate_user_account_data(
        &self,
        account: AccountId,
        pool_attributes: PoolAttributesForWithdrawValidation,
    ) -> Result<AccountData>;
    fn _balance_decrease_allowed(
        &self,
        pool_attributes: PoolAttributesForWithdrawValidation,
        account: AccountId,
        amount: Balance,
    ) -> Result<bool>;

    // event emission
    fn _emit_market_listed_event(&self, pool: AccountId);
    fn _emit_new_collateral_factor_event(
        &self,
        pool: AccountId,
        old: WrappedU256,
        new: WrappedU256,
    );
    fn _emit_pool_action_paused_event(&self, pool: AccountId, action: String, paused: bool);
    fn _emit_action_paused_event(&self, action: String, paused: bool);
    fn _emit_new_price_oracle_event(&self, old: Option<AccountId>, new: Option<AccountId>);
    fn _emit_new_flashloan_gateway_event(&self, _old: Option<AccountId>, _new: Option<AccountId>);
    fn _emit_new_close_factor_event(&self, old: WrappedU256, new: WrappedU256);
    fn _emit_new_liquidation_incentive_event(&self, old: WrappedU256, new: WrappedU256);
    fn _emit_new_borrow_cap_event(&self, pool: AccountId, new: Balance);
}

impl<T: Storage<Data>> Controller for T {
    default fn mint_allowed(
        &self,
        pool: AccountId,
        minter: AccountId,
        mint_amount: Balance,
    ) -> Result<()> {
        self._mint_allowed(pool, minter, mint_amount)
    }

    default fn mint_verify(
        &self,
        pool: AccountId,
        minter: AccountId,
        mint_amount: Balance,
        mint_tokens: Balance,
    ) -> Result<()> {
        self._mint_verify(pool, minter, mint_amount, mint_tokens)
    }

    default fn redeem_allowed(
        &self,
        pool: AccountId,
        redeemer: AccountId,
        redeem_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        self._redeem_allowed(pool, redeemer, redeem_amount, pool_attribute)
    }

    default fn redeem_verify(
        &self,
        pool: AccountId,
        redeemer: AccountId,
        redeem_amount: Balance,
    ) -> Result<()> {
        self._redeem_verify(pool, redeemer, redeem_amount)
    }

    default fn borrow_allowed(
        &self,
        pool: AccountId,
        borrower: AccountId,
        borrow_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        self._borrow_allowed(pool, borrower, borrow_amount, pool_attribute)
    }

    default fn borrow_verify(
        &self,
        pool: AccountId,
        borrower: AccountId,
        borrow_amount: Balance,
    ) -> Result<()> {
        self._borrow_verify(pool, borrower, borrow_amount)
    }

    default fn repay_borrow_allowed(
        &self,
        pool: AccountId,
        payer: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
    ) -> Result<()> {
        self._repay_borrow_allowed(pool, payer, borrower, repay_amount)
    }

    default fn repay_borrow_verify(
        &self,
        pool: AccountId,
        payer: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        borrower_index: u128,
    ) -> Result<()> {
        self._repay_borrow_verify(pool, payer, borrower, repay_amount, borrower_index)
    }

    default fn liquidate_borrow_allowed(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        self._liquidate_borrow_allowed(
            pool_borrowed,
            pool_collateral,
            liquidator,
            borrower,
            repay_amount,
            pool_attribute,
        )
    }

    default fn liquidate_borrow_verify(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        seize_tokens: Balance,
    ) -> Result<()> {
        self._liquidate_borrow_verify(
            pool_borrowed,
            pool_collateral,
            liquidator,
            borrower,
            repay_amount,
            seize_tokens,
        )
    }

    default fn seize_allowed(
        &self,
        pool_collateral: AccountId,
        pool_borrowed: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        seize_tokens: Balance,
    ) -> Result<()> {
        self._seize_allowed(
            pool_collateral,
            pool_borrowed,
            liquidator,
            borrower,
            seize_tokens,
        )
    }

    default fn seize_verify(
        &self,
        pool_collateral: AccountId,
        pool_borrowed: AccountId,
        liquidator: AccountId,
        borrower: AccountId,
        seize_tokens: Balance,
    ) -> Result<()> {
        self._seize_verify(
            pool_collateral,
            pool_borrowed,
            liquidator,
            borrower,
            seize_tokens,
        )
    }

    default fn transfer_allowed(
        &self,
        pool: AccountId,
        src: AccountId,
        dst: AccountId,
        transfer_tokens: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        self._transfer_allowed(pool, src, dst, transfer_tokens, pool_attribute)
    }

    default fn transfer_verify(
        &self,
        pool: AccountId,
        src: AccountId,
        dst: AccountId,
        transfer_tokens: Balance,
    ) -> Result<()> {
        self._transfer_verify(pool, src, dst, transfer_tokens)
    }

    default fn liquidate_calculate_seize_tokens(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        exchange_rate_mantissa: WrappedU256,
        repay_amount: Balance,
        pool_borrowed_attributes: Option<PoolAttributesForSeizeCalculation>,
        pool_collateral_attributes: Option<PoolAttributesForSeizeCalculation>,
    ) -> Result<Balance> {
        self._liquidate_calculate_seize_tokens(
            pool_borrowed,
            pool_collateral,
            exchange_rate_mantissa,
            repay_amount,
            pool_borrowed_attributes,
            pool_collateral_attributes,
        )
    }

    default fn set_price_oracle(&mut self, new_oracle: AccountId) -> Result<()> {
        self._assert_manager()?;
        let old = self._oracle();
        self._set_price_oracle(new_oracle)?;
        self._emit_new_price_oracle_event(old, Some(new_oracle));
        Ok(())
    }

    default fn support_market(&mut self, pool: AccountId, underlying: AccountId) -> Result<()> {
        self._assert_manager()?;
        self._support_market(&pool, &underlying, None)?;
        self._emit_market_listed_event(pool);
        Ok(())
    }

    default fn set_flashloan_gateway(&mut self, new_flashloan_gateway: AccountId) -> Result<()> {
        self._assert_manager()?;
        let old = self._flashloan_gateway();
        self._set_flashloan_gateway(new_flashloan_gateway)?;
        self._emit_new_flashloan_gateway_event(old, Some(new_flashloan_gateway));
        Ok(())
    }

    default fn support_market_with_collateral_factor_mantissa(
        &mut self,
        pool: AccountId,
        underlying: AccountId,
        collateral_factor_mantissa: WrappedU256,
    ) -> Result<()> {
        self._assert_manager()?;
        self._support_market(&pool, &underlying, Some(collateral_factor_mantissa))?;
        self._emit_market_listed_event(pool);
        Ok(())
    }

    default fn set_collateral_factor_mantissa(
        &mut self,
        pool: AccountId,
        new_collateral_factor_mantissa: WrappedU256,
    ) -> Result<()> {
        self._assert_manager()?;
        let old = self._collateral_factor_mantissa(pool).unwrap_or_default();
        self._set_collateral_factor_mantissa(&pool, new_collateral_factor_mantissa)?;
        self._emit_new_collateral_factor_event(pool, old, new_collateral_factor_mantissa);
        Ok(())
    }

    default fn set_mint_guardian_paused(&mut self, pool: AccountId, paused: bool) -> Result<()> {
        self._assert_manager()?;
        self._set_mint_guardian_paused(&pool, paused)?;
        self._emit_pool_action_paused_event(pool, String::from("Mint"), paused);
        Ok(())
    }

    default fn set_borrow_guardian_paused(&mut self, pool: AccountId, paused: bool) -> Result<()> {
        self._assert_manager()?;
        self._set_borrow_guardian_paused(&pool, paused)?;
        self._emit_pool_action_paused_event(pool, String::from("Borrow"), paused);
        Ok(())
    }

    default fn set_seize_guardian_paused(&mut self, paused: bool) -> Result<()> {
        self._assert_manager()?;
        self._set_seize_guardian_paused(paused)?;
        self._emit_action_paused_event(String::from("Seize"), paused);
        Ok(())
    }

    default fn set_transfer_guardian_paused(&mut self, paused: bool) -> Result<()> {
        self._assert_manager()?;
        self._set_transfer_guardian_paused(paused)?;
        self._emit_action_paused_event(String::from("Transfer"), paused);
        Ok(())
    }

    default fn set_close_factor_mantissa(
        &mut self,
        new_close_factor_mantissa: WrappedU256,
    ) -> Result<()> {
        self._assert_manager()?;
        let old = self._close_factor_mantissa();
        self._set_close_factor_mantissa(new_close_factor_mantissa)?;
        self._emit_new_close_factor_event(old, new_close_factor_mantissa);
        Ok(())
    }

    default fn set_liquidation_incentive_mantissa(
        &mut self,
        new_liquidation_incentive_mantissa: WrappedU256,
    ) -> Result<()> {
        self._assert_manager()?;
        let old = self._liquidation_incentive_mantissa();
        self._set_liquidation_incentive_mantissa(new_liquidation_incentive_mantissa)?;
        self._emit_new_liquidation_incentive_event(old, new_liquidation_incentive_mantissa);
        Ok(())
    }

    default fn set_borrow_cap(&mut self, pool: AccountId, new_cap: Balance) -> Result<()> {
        self._assert_manager()?;
        self._set_borrow_cap(&pool, new_cap)?;
        self._emit_new_borrow_cap_event(pool, new_cap);
        Ok(())
    }

    default fn markets(&self) -> Vec<AccountId> {
        self._markets()
    }
    default fn market_of_underlying(&self, underlying: AccountId) -> Option<AccountId> {
        self._market_of_underlying(underlying)
    }
    default fn flashloan_gateway(&self) -> Option<AccountId> {
        self._flashloan_gateway()
    }
    default fn collateral_factor_mantissa(&self, pool: AccountId) -> Option<WrappedU256> {
        self._collateral_factor_mantissa(pool)
    }
    default fn mint_guardian_paused(&self, pool: AccountId) -> Option<bool> {
        self._mint_guardian_paused(pool)
    }
    default fn borrow_guardian_paused(&self, pool: AccountId) -> Option<bool> {
        self._borrow_guardian_paused(pool)
    }
    default fn seize_guardian_paused(&self) -> bool {
        self._seize_guardian_paused()
    }
    default fn transfer_guardian_paused(&self) -> bool {
        self._transfer_guardian_paused()
    }
    default fn oracle(&self) -> Option<AccountId> {
        self._oracle()
    }
    default fn close_factor_mantissa(&self) -> WrappedU256 {
        self._close_factor_mantissa()
    }
    default fn liquidation_incentive_mantissa(&self) -> WrappedU256 {
        self._liquidation_incentive_mantissa()
    }
    default fn borrow_cap(&self, pool: AccountId) -> Option<Balance> {
        self._borrow_cap(pool)
    }
    default fn manager(&self) -> Option<AccountId> {
        self._manager()
    }
    default fn is_listed(&self, pool: AccountId) -> bool {
        self._is_listed(pool)
    }
    default fn account_assets(&self, account: AccountId) -> Vec<AccountId> {
        self._account_assets(account, None)
    }
    default fn get_account_liquidity(&self, account: AccountId) -> Result<(U256, U256)> {
        self._get_account_liquidity(account)
    }
    default fn get_hypothetical_account_liquidity(
        &self,
        account: AccountId,
        token: AccountId,
        redeem_tokens: Balance,
        borrow_amount: Balance,
    ) -> Result<(U256, U256)> {
        self._get_hypothetical_account_liquidity(
            account,
            Some(token),
            redeem_tokens,
            borrow_amount,
            None,
        )
    }

    default fn calculate_user_account_data(
        &self,
        account: AccountId,
        pool_attributes: PoolAttributesForWithdrawValidation,
    ) -> Result<AccountData> {
        self._calculate_user_account_data(account, pool_attributes)
    }

    default fn balance_decrease_allowed(
        &self,
        pool_attributes: PoolAttributesForWithdrawValidation,
        account: AccountId,
        amount: Balance,
    ) -> Result<bool> {
        self._balance_decrease_allowed(pool_attributes, account, amount)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _mint_allowed(
        &self,
        pool: AccountId,
        _minter: AccountId,
        _mint_amount: Balance,
    ) -> Result<()> {
        if let Some(true) | None = self._mint_guardian_paused(pool) {
            return Err(Error::MintIsPaused)
        }

        // FEATURE: update governance token supply index & distribute

        Ok(())
    }
    default fn _mint_verify(
        &self,
        _pool: AccountId,
        _minter: AccountId,
        _mint_amount: Balance,
        _mint_tokens: Balance,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _redeem_allowed(
        &self,
        pool: AccountId,
        redeemer: AccountId,
        redeem_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        let caller_pool = if pool_attribute.is_some() {
            Some((pool, pool_attribute.unwrap()))
        } else {
            None
        };
        let (_, shortfall) = self._get_hypothetical_account_liquidity(
            redeemer,
            Some(pool),
            redeem_amount,
            0,
            caller_pool,
        )?;
        if !shortfall.is_zero() {
            return Err(Error::InsufficientLiquidity)
        }

        // FEATURE: update governance token supply index & distribute

        Ok(())
    }
    default fn _redeem_verify(
        &self,
        _pool: AccountId,
        _redeemer: AccountId,
        _amount: Balance,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _borrow_allowed(
        &self,
        pool: AccountId,
        borrower: AccountId,
        borrow_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        if let Some(true) | None = self._borrow_guardian_paused(pool) {
            return Err(Error::BorrowIsPaused)
        }

        let oracle = self._oracle();
        if oracle.is_none() {
            return Err(Error::OracleIsNotSet)
        }
        let _oracle = oracle.unwrap();

        let (price, total_borrow, caller_pool) = if pool_attribute.is_none() {
            (
                PriceOracleRef::get_underlying_price(&_oracle, pool),
                PoolRef::total_borrows(&pool),
                None,
            )
        } else {
            let attrs = pool_attribute.unwrap();
            if attrs.underlying.is_none() {
                return Err(Error::UnderlyingIsNotSet)
            }

            (
                PriceOracleRef::get_price(&_oracle, attrs.underlying.unwrap()),
                attrs.total_borrows,
                Some((pool, attrs)),
            )
        };
        if let None | Some(0) = price {
            return Err(Error::PriceError)
        }
        let borrow_cap = self._borrow_cap(pool).unwrap();
        if borrow_cap != 0 {
            if borrow_cap < borrow_amount || total_borrow > borrow_cap - borrow_amount {
                return Err(Error::BorrowCapReached)
            }
        }

        let (_, shortfall) = self._get_hypothetical_account_liquidity(
            borrower,
            Some(pool),
            0,
            borrow_amount,
            caller_pool,
        )?;
        if !shortfall.is_zero() {
            return Err(Error::InsufficientLiquidity)
        }

        // FEATURE: update governance token borrow index & distribute

        Ok(())
    }
    default fn _borrow_verify(
        &self,
        _pool: AccountId,
        _borrower: AccountId,
        _borrow_amount: Balance,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _repay_borrow_allowed(
        &self,
        _pool: AccountId,
        _payer: AccountId,
        _borrower: AccountId,
        _repay_amount: Balance,
    ) -> Result<()> {
        // FEATURE: update governance token borrow index & distribute

        Ok(())
    }
    default fn _repay_borrow_verify(
        &self,
        _pool: AccountId,
        _payer: AccountId,
        _borrower: AccountId,
        _repay_amount: Balance,
        _borrower_index: u128,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _liquidate_borrow_allowed(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        _liquidator: AccountId,
        borrower: AccountId,
        repay_amount: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        if !self._is_listed(pool_borrowed) || !self._is_listed(pool_collateral) {
            return Err(Error::MarketNotListed)
        }

        let (caller_pool, borrow_balance) = if pool_attribute.is_some() {
            let attrs = pool_attribute.unwrap();
            (
                Some((pool_borrowed, attrs.clone())),
                attrs.account_borrow_balance,
            )
        } else {
            (
                None,
                PoolRef::borrow_balance_stored(&pool_borrowed, borrower),
            )
        };

        // The borrower must have shortfall in order to be liquidatable
        let (_, shortfall) =
            self._get_hypothetical_account_liquidity(borrower, None, 0, 0, caller_pool)?;
        if shortfall.is_zero() {
            return Err(Error::InsufficientShortfall)
        }

        // The liquidator may not repay more than what is allowed by the closeFactor
        let max_close = Exp {
            mantissa: self._close_factor_mantissa(),
        }
        .mul_scalar_truncate(U256::from(borrow_balance));
        if U256::from(repay_amount).gt(&max_close) {
            return Err(Error::TooMuchRepay)
        }

        Ok(())
    }
    default fn _liquidate_borrow_verify(
        &self,
        _pool_borrowed: AccountId,
        _pool_collateral: AccountId,
        _liquidator: AccountId,
        _borrower: AccountId,
        _repay_amount: Balance,
        _seize_tokens: Balance,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _seize_allowed(
        &self,
        pool_collateral: AccountId,
        pool_borrowed: AccountId,
        _liquidator: AccountId,
        _borrower: AccountId,
        _seize_tokens: Balance,
    ) -> Result<()> {
        if self._seize_guardian_paused() {
            return Err(Error::SeizeIsPaused)
        }

        if !self._is_listed(pool_collateral) || !self._is_listed(pool_borrowed) {
            return Err(Error::MarketNotListed)
        }

        // NOTE: cannot perform controller check on the pool here, as a cross-contract call to the caller occurs when the pool is the caller.
        //   To avoid this, the pool itself needs to perform this check.
        // let p_collateral_ctrler = PoolRef::controller(&pool_collateral);
        // let p_borrowed_ctrler = PoolRef::controller(&pool_borrowed);
        // if p_collateral_ctrler != p_borrowed_ctrler {
        //     return Err(Error::ControllerMismatch)
        // }

        // FEATURE: update governance token supply index & distribute to borrower,liquidator

        Ok(())
    }
    default fn _seize_verify(
        &self,
        _pool_collateral: AccountId,
        _pool_borrowed: AccountId,
        _liquidator: AccountId,
        _borrower: AccountId,
        _seize_tokens: Balance,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _transfer_allowed(
        &self,
        pool: AccountId,
        src: AccountId,
        _dst: AccountId,
        transfer_tokens: Balance,
        pool_attribute: Option<PoolAttributes>,
    ) -> Result<()> {
        if self._transfer_guardian_paused() {
            return Err(Error::TransferIsPaused)
        }

        self._redeem_allowed(pool, src, transfer_tokens, pool_attribute)?;

        // FEATURE: update governance token supply index & distribute

        Ok(())
    }
    default fn _transfer_verify(
        &self,
        _pool: AccountId,
        _src: AccountId,
        _dst: AccountId,
        _transfer_tokens: Balance,
    ) -> Result<()> {
        Ok(()) // do nothing
    }
    default fn _liquidate_calculate_seize_tokens(
        &self,
        pool_borrowed: AccountId,
        pool_collateral: AccountId,
        exchange_rate_mantissa: WrappedU256,
        repay_amount: Balance,
        pool_borrowed_attributes: Option<PoolAttributesForSeizeCalculation>,
        pool_collateral_attributes: Option<PoolAttributesForSeizeCalculation>,
    ) -> Result<Balance> {
        let oracle = self._oracle();
        if oracle.is_none() {
            return Err(Error::OracleIsNotSet)
        }
        let _oracle = oracle.unwrap();

        let (price_borrowed_mantissa, pool_decimals_borrowed) =
            if let Some(attrs) = pool_borrowed_attributes {
                if attrs.underlying.is_none() {
                    return Err(Error::UnderlyingIsNotSet)
                }
                (
                    PriceOracleRef::get_price(&_oracle, attrs.underlying.unwrap()),
                    attrs.decimals,
                )
            } else {
                (
                    PriceOracleRef::get_underlying_price(&_oracle, pool_borrowed),
                    PoolRef::token_decimals(&pool_borrowed),
                )
            };
        if let None | Some(0) = price_borrowed_mantissa {
            return Err(Error::PriceError)
        }

        let (price_collateral_mantissa, pool_decimals_collateral) =
            if let Some(attrs) = pool_collateral_attributes {
                if attrs.underlying.is_none() {
                    return Err(Error::UnderlyingIsNotSet)
                }
                (
                    PriceOracleRef::get_price(&_oracle, attrs.underlying.unwrap()),
                    attrs.decimals,
                )
            } else {
                (
                    PriceOracleRef::get_underlying_price(&_oracle, pool_collateral),
                    PoolRef::token_decimals(&pool_collateral),
                )
            };
        if let None | Some(0) = price_collateral_mantissa {
            return Err(Error::PriceError)
        }

        let result = liquidate_calculate_seize_tokens(&LiquidateCalculateSeizeTokensInput {
            price_borrowed_mantissa: U256::from(price_borrowed_mantissa.unwrap()),
            decimals_borrowed: pool_decimals_borrowed,
            price_collateral_mantissa: U256::from(price_collateral_mantissa.unwrap()),
            decimals_collateral: pool_decimals_collateral,
            exchange_rate_mantissa: exchange_rate_mantissa.into(),
            liquidation_incentive_mantissa: self._liquidation_incentive_mantissa().into(),
            actual_repay_amount: repay_amount,
        });
        Ok(result)
    }
    default fn _assert_manager(&self) -> Result<()> {
        let manager = self._manager();
        if manager.is_none() {
            return Err(Error::ManagerIsNotSet)
        }
        let _manager = manager.unwrap();
        if Self::env().caller() != _manager {
            return Err(Error::CallerIsNotManager)
        }
        Ok(())
    }
    default fn _set_price_oracle(&mut self, new_oracle: AccountId) -> Result<()> {
        self.data().oracle = Some(new_oracle);
        Ok(())
    }
    default fn _set_flashloan_gateway(&mut self, new_flashloan_gateway: AccountId) -> Result<()> {
        self.data().flashloan_gateway = Some(new_flashloan_gateway);
        Ok(())
    }
    default fn _support_market(
        &mut self,
        pool: &AccountId,
        underlying: &AccountId,
        collateral_factor_mantissa: Option<WrappedU256>,
    ) -> Result<()> {
        for market in self._markets() {
            if pool == &market {
                return Err(Error::MarketAlreadyListed)
            }
        }

        self.data().markets.push(*pool);
        self.data().markets_pair.insert(underlying, pool);

        // set default states
        self._set_mint_guardian_paused(pool, false)?;
        self._set_borrow_guardian_paused(pool, false)?;
        if let Some(value) = collateral_factor_mantissa {
            self._set_collateral_factor_mantissa(pool, value)?;
        }
        self._set_borrow_cap(pool, 0)?;

        Ok(())
    }
    default fn _set_collateral_factor_mantissa(
        &mut self,
        pool: &AccountId,
        new_collateral_factor_mantissa: WrappedU256,
    ) -> Result<()> {
        let new_collateral_factor_mantissa_u256 = U256::from(new_collateral_factor_mantissa);
        if new_collateral_factor_mantissa_u256.is_zero()
            || new_collateral_factor_mantissa_u256.gt(&collateral_factor_max_mantissa())
        {
            return Err(Error::InvalidCollateralFactor)
        }

        let oracle = self._oracle();
        if oracle.is_none() {
            return Err(Error::OracleIsNotSet)
        }
        let _oracle = oracle.unwrap();
        if let None | Some(0) = PriceOracleRef::get_underlying_price(&_oracle, *pool) {
            return Err(Error::PriceError)
        }

        self.data()
            .collateral_factor_mantissa
            .insert(pool, &new_collateral_factor_mantissa);
        Ok(())
    }
    default fn _set_mint_guardian_paused(&mut self, pool: &AccountId, paused: bool) -> Result<()> {
        self.data().mint_guardian_paused.insert(pool, &paused);
        Ok(())
    }
    default fn _set_borrow_guardian_paused(
        &mut self,
        pool: &AccountId,
        paused: bool,
    ) -> Result<()> {
        self.data().borrow_guardian_paused.insert(pool, &paused);
        Ok(())
    }
    default fn _set_seize_guardian_paused(&mut self, paused: bool) -> Result<()> {
        self.data().seize_guardian_paused = paused;
        Ok(())
    }
    default fn _set_transfer_guardian_paused(&mut self, paused: bool) -> Result<()> {
        self.data().transfer_guardian_paused = paused;
        Ok(())
    }
    default fn _set_close_factor_mantissa(
        &mut self,
        new_close_factor_mantissa: WrappedU256,
    ) -> Result<()> {
        self.data().close_factor_mantissa = new_close_factor_mantissa;
        Ok(())
    }
    default fn _set_liquidation_incentive_mantissa(
        &mut self,
        new_liquidation_incentive_mantissa: WrappedU256,
    ) -> Result<()> {
        self.data().liquidation_incentive_mantissa = new_liquidation_incentive_mantissa;
        Ok(())
    }
    default fn _set_borrow_cap(&mut self, pool: &AccountId, new_cap: Balance) -> Result<()> {
        self.data().borrow_caps.insert(pool, &new_cap);
        Ok(())
    }

    default fn _markets(&self) -> Vec<AccountId> {
        self.data().markets.clone()
    }
    default fn _market_of_underlying(&self, underlying: AccountId) -> Option<AccountId> {
        self.data().markets_pair.get(&underlying)
    }
    default fn _flashloan_gateway(&self) -> Option<AccountId> {
        self.data().flashloan_gateway
    }
    default fn _is_listed(&self, pool: AccountId) -> bool {
        let markets = self._markets();
        for market in markets {
            if market == pool {
                return true
            }
        }
        return false
    }
    default fn _collateral_factor_mantissa(&self, pool: AccountId) -> Option<WrappedU256> {
        self.data().collateral_factor_mantissa.get(&pool)
    }
    default fn _mint_guardian_paused(&self, pool: AccountId) -> Option<bool> {
        self.data().mint_guardian_paused.get(&pool)
    }
    default fn _borrow_guardian_paused(&self, pool: AccountId) -> Option<bool> {
        self.data().borrow_guardian_paused.get(&pool)
    }
    default fn _seize_guardian_paused(&self) -> bool {
        self.data().seize_guardian_paused
    }
    default fn _transfer_guardian_paused(&self) -> bool {
        self.data().transfer_guardian_paused
    }
    default fn _oracle(&self) -> Option<AccountId> {
        self.data().oracle
    }
    default fn _close_factor_mantissa(&self) -> WrappedU256 {
        self.data::<Data>().close_factor_mantissa
    }
    default fn _liquidation_incentive_mantissa(&self) -> WrappedU256 {
        self.data::<Data>().liquidation_incentive_mantissa
    }
    default fn _borrow_cap(&self, pool: AccountId) -> Option<Balance> {
        self.data().borrow_caps.get(&pool)
    }
    default fn _manager(&self) -> Option<AccountId> {
        self.data().manager
    }

    default fn _account_assets(
        &self,
        account: AccountId,
        token_modify: Option<AccountId>,
    ) -> Vec<AccountId> {
        let mut account_assets = Vec::<AccountId>::new();
        let markets = self._markets();
        for pool in markets {
            if pool == Self::env().caller() {
                continue // NOTE: if caller is pool, need to check by the pool itself
            }
            if token_modify.is_some() && pool == token_modify.unwrap() {
                account_assets.push(pool); // NOTE: add unconditionally even if balance, borrowed is not already there
                continue
            }
            let (balance, borrowed, _) = PoolRef::get_account_snapshot(&pool, account);

            // whether deposits or loans exist
            if balance > 0 || borrowed > 0 {
                account_assets.push(pool);
            }
        }
        return account_assets
    }

    default fn _get_account_liquidity(&self, account: AccountId) -> Result<(U256, U256)> {
        self._get_hypothetical_account_liquidity(account, None, 0, 0, None)
    }

    default fn _get_hypothetical_account_liquidity(
        &self,
        account: AccountId,
        token_modify: Option<AccountId>,
        redeem_tokens: Balance,
        borrow_amount: Balance,
        caller_pool: Option<(AccountId, PoolAttributes)>,
    ) -> Result<(U256, U256)> {
        // For each asset the account is in
        let account_assets = self._account_assets(account, token_modify);
        let mut asset_params = Vec::<HypotheticalAccountLiquidityCalculationParam>::new();

        let oracle = self._oracle();
        if oracle.is_none() {
            return Err(Error::OracleIsNotSet)
        }
        let _oracle = oracle.unwrap();

        // if caller is a pool, get parameters for the pool without call the pool
        if let Some((caller_pool_id, attrs)) = caller_pool {
            if attrs.underlying.is_none() {
                return Err(Error::UnderlyingIsNotSet)
            }
            let oracle_price = PriceOracleRef::get_price(&_oracle, attrs.underlying.unwrap());
            if let None | Some(0) = oracle_price {
                return Err(Error::PriceError)
            }
            let oracle_price_mantissa = Exp {
                mantissa: WrappedU256::from(U256::from(oracle_price.clone().unwrap())),
            };

            asset_params.push(HypotheticalAccountLiquidityCalculationParam {
                asset: caller_pool_id,
                decimals: attrs.decimals,
                token_balance: attrs.account_balance,
                borrow_balance: attrs.account_borrow_balance,
                exchange_rate_mantissa: Exp {
                    mantissa: WrappedU256::from(attrs.exchange_rate),
                },
                collateral_factor_mantissa: Exp {
                    mantissa: self._collateral_factor_mantissa(caller_pool_id).unwrap(),
                },
                oracle_price_mantissa: oracle_price_mantissa.clone(),
            })
        }

        // Prepare parameters for calculation
        for asset in &account_assets {
            // Read the balances and exchange rate from the pool
            let (token_balance, borrow_balance, exchange_rate_mantissa) =
                PoolRef::get_account_snapshot(asset, account);
            let decimals = PoolRef::token_decimals(asset);

            // Get the normalized price of the asset
            let oracle_price = PriceOracleRef::get_underlying_price(&_oracle, *asset);
            if let None | Some(0) = oracle_price {
                return Err(Error::PriceError)
            }
            let oracle_price_mantissa = Exp {
                mantissa: WrappedU256::from(U256::from(oracle_price.clone().unwrap())),
            };

            asset_params.push(HypotheticalAccountLiquidityCalculationParam {
                asset: *asset,
                decimals,
                token_balance,
                borrow_balance,
                exchange_rate_mantissa: Exp {
                    mantissa: WrappedU256::from(exchange_rate_mantissa),
                },
                collateral_factor_mantissa: Exp {
                    mantissa: self._collateral_factor_mantissa(*asset).unwrap(),
                },
                oracle_price_mantissa: oracle_price_mantissa.clone(),
            });
        }

        let (sum_collateral, sum_borrow_plus_effect) =
            get_hypothetical_account_liquidity(GetHypotheticalAccountLiquidityInput {
                asset_params,
                token_modify,
                redeem_tokens,
                borrow_amount,
            });

        // These are safe, as the underflow condition is checked first
        let value = if sum_collateral > sum_borrow_plus_effect {
            (sum_collateral.sub(sum_borrow_plus_effect), U256::from(0))
        } else {
            (U256::from(0), sum_borrow_plus_effect.sub(sum_collateral))
        };
        Ok(value)
    }

    default fn _calculate_user_account_data(
        &self,
        account: AccountId,
        pool_attributes: PoolAttributesForWithdrawValidation,
    ) -> Result<AccountData> {
        let account_assets: Vec<AccountId> = self.account_assets(account);

        let mut total_collateral_in_base_currency = U256::from(0);
        let mut avg_ltv = U256::from(0);
        let mut avg_liquidation_threshold = U256::from(0);
        let mut total_debt_in_base_currency: U256 = U256::from(0);

        let oracle = self._oracle();
        if oracle.is_none() {
            return Err(Error::OracleIsNotSet)
        }
        let _oracle = oracle.unwrap();

        if pool_attributes.pool.is_none() {
            return Err(Error::PoolIsNotSet)
        }

        let collateral_factor_mantissa: Option<WrappedU256> =
            self.collateral_factor_mantissa(pool_attributes.pool.unwrap());
        if collateral_factor_mantissa.is_none() {
            return Err(Error::MarketNotListed)
        }
        let ltv = U256::from(collateral_factor_mantissa.unwrap());

        let liquidation_threshold = pool_attributes.liquidation_threshold;

        if pool_attributes.underlying.is_none() {
            return Err(Error::UnderlyingIsNotSet)
        }
        let unit_price_result =
            PriceOracleRef::get_price(&_oracle, pool_attributes.underlying.unwrap());
        if unit_price_result.is_none() {
            return Err(Error::PriceError)
        }
        let unit_price = unit_price_result.unwrap();
        let compounded_liquidity_balance = pool_attributes.account_balance;
        let borrow_balance_stored = pool_attributes.account_borrow_balance;

        if compounded_liquidity_balance != 0 {
            let liquidity_balance_eth = U256::from(unit_price)
                .mul(U256::from(compounded_liquidity_balance))
                .div(U256::from(PRICE_PRECISION));
            total_collateral_in_base_currency =
                total_collateral_in_base_currency.add(liquidity_balance_eth);
            avg_ltv = avg_ltv.add(liquidity_balance_eth.mul(U256::from(ltv)));
            avg_liquidation_threshold = avg_liquidation_threshold
                .add(liquidity_balance_eth.mul(U256::from(liquidation_threshold)));
        }

        if borrow_balance_stored != 0 {
            let borrow_balance_eth = U256::from(unit_price)
                .mul(U256::from(borrow_balance_stored))
                .div(U256::from(PRICE_PRECISION));
            total_debt_in_base_currency = total_debt_in_base_currency.add(borrow_balance_eth);
        }

        for asset in account_assets {
            let collateral_factor_mantissa: Option<WrappedU256> =
                self.collateral_factor_mantissa(asset);
            if collateral_factor_mantissa.is_none() {
                return Err(Error::MarketNotListed)
            }
            let ltv = U256::from(collateral_factor_mantissa.unwrap());

            let liquidation_threshold = PoolRef::liquidation_threshold(&asset);

            let underlying = PoolRef::underlying(&asset);
            if underlying.is_none() {
                return Err(Error::UnderlyingIsNotSet)
            }

            let unit_price_result = PriceOracleRef::get_price(&_oracle, underlying.unwrap());
            if unit_price_result.is_none() {
                return Err(Error::PriceError)
            }
            let unit_price = unit_price_result.unwrap();
            let (compounded_liquidity_balance, borrow_balance_stored, _) =
                PoolRef::get_account_snapshot(&asset, account);

            if compounded_liquidity_balance != 0 {
                let liquidity_balance_eth = U256::from(unit_price)
                    .mul(U256::from(compounded_liquidity_balance))
                    .div(U256::from(PRICE_PRECISION));
                total_collateral_in_base_currency =
                    total_collateral_in_base_currency.add(liquidity_balance_eth);
                avg_ltv = avg_ltv.add(liquidity_balance_eth.mul(U256::from(ltv)));
                avg_liquidation_threshold = avg_liquidation_threshold
                    .add(liquidity_balance_eth.mul(U256::from(liquidation_threshold)));
            }

            if borrow_balance_stored != 0 {
                let borrow_balance_eth = U256::from(unit_price)
                    .mul(U256::from(borrow_balance_stored))
                    .div(U256::from(PRICE_PRECISION));
                total_debt_in_base_currency = total_debt_in_base_currency.add(borrow_balance_eth);
            }
        }

        avg_ltv = if total_collateral_in_base_currency.is_zero() {
            U256::from(0)
        } else {
            avg_ltv.div(total_collateral_in_base_currency)
        };

        avg_liquidation_threshold = if total_collateral_in_base_currency.is_zero() {
            U256::from(0)
        } else {
            avg_liquidation_threshold.div(total_collateral_in_base_currency)
        };

        let health_factor = calculate_health_factor_from_balances(
            total_collateral_in_base_currency,
            total_debt_in_base_currency,
            avg_liquidation_threshold,
        );
        Ok(AccountData {
            total_collateral_in_base_currency,
            total_debt_in_base_currency,
            avg_ltv,
            avg_liquidation_threshold,
            health_factor,
        })
    }

    default fn _balance_decrease_allowed(
        &self,
        pool_attributes: PoolAttributesForWithdrawValidation,
        account: AccountId,
        amount: Balance,
    ) -> Result<bool> {
        let oracle = self._oracle();
        if oracle.is_none() {
            return Err(Error::OracleIsNotSet)
        }
        let _oracle = oracle.unwrap();

        let account_data = self._calculate_user_account_data(account, pool_attributes.clone())?;

        let total_debt_in_base_currency = account_data.total_debt_in_base_currency;

        if total_debt_in_base_currency.is_zero() {
            return Ok(true)
        }

        if pool_attributes.underlying.is_none() {
            return Err(Error::UnderlyingIsNotSet)
        }

        let asset_price = PriceOracleRef::get_price(&_oracle, pool_attributes.underlying.unwrap());
        if let None | Some(0) = asset_price {
            return Ok(false)
        }

        return Ok(utils::balance_decrease_allowed(
            BalanceDecreaseAllowedParam {
                total_collateral_in_base_currency: account_data.total_collateral_in_base_currency,
                total_debt_in_base_currency,
                avg_liquidation_threshold: account_data.avg_liquidation_threshold,
                amount_in_base_currency_unit: amount.into(),
                asset_price: asset_price.unwrap().into(),
                liquidation_threshold: pool_attributes.liquidation_threshold.into(),
            },
        ))
    }

    default fn _emit_market_listed_event(&self, _pool: AccountId) {}
    default fn _emit_new_collateral_factor_event(
        &self,
        _pool: AccountId,
        _old: WrappedU256,
        _new: WrappedU256,
    ) {
    }
    default fn _emit_pool_action_paused_event(
        &self,
        _pool: AccountId,
        _action: String,
        _paused: bool,
    ) {
    }
    default fn _emit_action_paused_event(&self, _action: String, _paused: bool) {}
    default fn _emit_new_price_oracle_event(
        &self,
        _old: Option<AccountId>,
        _new: Option<AccountId>,
    ) {
    }
    default fn _emit_new_flashloan_gateway_event(
        &self,
        _old: Option<AccountId>,
        _new: Option<AccountId>,
    ) {
    }
    default fn _emit_new_close_factor_event(&self, _old: WrappedU256, _new: WrappedU256) {}
    default fn _emit_new_liquidation_incentive_event(&self, _old: WrappedU256, _new: WrappedU256) {}
    default fn _emit_new_borrow_cap_event(&self, _pool: AccountId, _new: Balance) {}
}
