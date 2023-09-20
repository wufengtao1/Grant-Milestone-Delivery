// SPDX-License-Identifier: MIT
import { after, describe } from 'mocha'

import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import {SECURITY_PREFIX} from "../shared/consts";
import { Signers } from '../shared/signers'
import { setupUser as setup } from '../shared/test-setups/user'

describe(SECURITY_PREFIX + 'User', () => {
  it('Cannot set user data if not owner', async () => {
    const contract = await setup()

    await expect(contract.query.getUserData()).to.have.deepReturnValue({
      nick: null,
      avatar: null,
      additionInfo: null,
    })

    await expect(
      contract.withSigner(Signers.Alice).tx.setUserData({
        nick: '@some_nick',
        avatar: null,
        additionInfo: 'Some additional info',
      }),
    ).to.be.rejected

    await expect(contract.query.getUserData()).to.have.deepReturnValue({
      nick: null,
      avatar: null,
      additionInfo: null,
    })
  })

  after(async () => {
    await ApiSingleton.disconnect()
  })
})
