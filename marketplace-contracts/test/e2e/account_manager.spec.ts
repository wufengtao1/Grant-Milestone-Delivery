// SPDX-License-Identifier: MIT
import { after, describe } from 'mocha'

import CreatorABI from '../../artifacts/creator.json'
import UserABI from '../../artifacts/user.json'
import ContractUser from '../../typechain-generated/contracts/user'
import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import {E2E_PREFIX} from "../shared/consts";
import { Signers } from '../shared/signers'
import { setupAccountManager as setup } from '../shared/test-setups/account_manager'


describe(E2E_PREFIX + 'Account Manager', () => {
  const USER_CODE_HASH = UserABI.source.hash
  const CREATOR_CODE_HASH = CreatorABI.source.hash

  it('Can create user and creator account', async () => {
    const contract = await setup()

    await contract.tx.createAccount()
    await contract.tx.createCreatorAccount()
  })

  it('Deploys real contract', async () => {
    const contract = await setup()

    const defaultSigner = Signers.defaultSigner

    await contract.tx.createAccount()

    const account = (await contract.query.getAccount(defaultSigner.address)).value.unwrapRecursively()!

    const accountContract = new ContractUser(
      account as unknown as string,
      defaultSigner,
      await ApiSingleton.getInstance(),
    )

    await expect(accountContract.query.owner()).to.have.returnValue(defaultSigner.address)
  })

  it('Admin can change creator code hash', async () => {
    const contract = await setup()

    await expect(contract.tx.setCreatorCodeHash(USER_CODE_HASH)).to.eventually.be.fulfilled
  })

  it('Admin can change user code hash', async () => {
    const contract = await setup()

    await expect(contract.tx.setUserCodeHash(CREATOR_CODE_HASH)).to.eventually.be.fulfilled
  })

  after(async () => {
    await ApiSingleton.disconnect()
  })
})
