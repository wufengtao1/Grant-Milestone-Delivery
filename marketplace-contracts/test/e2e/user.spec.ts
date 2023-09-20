// SPDX-License-Identifier: MIT
import { after, describe } from 'mocha'

import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import {E2E_PREFIX} from "../shared/consts";
import { setupUser as setup } from '../shared/test-setups/user'

describe(E2E_PREFIX + 'User', () => {
  it('Can set user data', async () => {
    const contract = await setup()

    await expect(contract.query.getUserData()).to.have.deepReturnValue({
      nick: null,
      avatar: null,
      additionInfo: null,
    })

    await contract.tx.setUserData({
      nick: '@some_nick',
      avatar: null,
      additionInfo: 'Some additional info',
    })

    await expect(contract.query.getUserData()).to.have.deepReturnValue({
      nick: '@some_nick',
      avatar: null,
      additionInfo: 'Some additional info',
    })
  })

  after(async () => {
    await ApiSingleton.disconnect()
  })
})
