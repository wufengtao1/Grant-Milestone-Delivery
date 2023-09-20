// SPDX-License-Identifier: MIT
import BN from 'bn.js'
import { describe } from 'mocha'

import CreatorABI from '../../artifacts/creator.json'
import UserABI from '../../artifacts/user.json'
import Contract from '../../typechain-generated/contracts/account_manager'
import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import { PERFORMANCE_PREFIX } from '../shared/consts'
import { setupAccountManager } from '../shared/test-setups/account_manager'

const CREATE_ACCOUNT_MAX_FEE = new BN(13_500_000_000)
const CREATE_CREATOR_ACCOUNT_MAX_FEE = new BN(13_100_000_000)
const SET_CREATOR_CODE_HASH_MAX_FEE = new BN(10_000_000_000)
const SET_USER_CODE_HASH_MAX_FEE = new BN(10_000_000_000)

describe(PERFORMANCE_PREFIX + 'AccountManager', function () {
  let contract: Contract

  beforeEach(async function () {
    contract = await setupAccountManager()
  })

  after(async function () {
    await ApiSingleton.disconnect()
  })

  it('Should create an account within max fee', async function () {
    await expect(contract.query.createAccount()).to.have.feeLessThan(CREATE_ACCOUNT_MAX_FEE)
  })

  it('Should create a creator account within max fee', async function () {
    await expect(contract.query.createCreatorAccount()).to.have.feeLessThan(CREATE_CREATOR_ACCOUNT_MAX_FEE)
  })

  it('Should set the creator code hash within max fee', async function () {
    await expect(contract.query.setCreatorCodeHash(CreatorABI.source.hash)).to.have.feeLessThan(
      SET_CREATOR_CODE_HASH_MAX_FEE,
    )
  })

  it('Should set the user code hash within max fee', async function () {
    await expect(contract.query.setUserCodeHash(UserABI.source.hash)).to.have.feeLessThan(SET_USER_CODE_HASH_MAX_FEE)
  })
})
