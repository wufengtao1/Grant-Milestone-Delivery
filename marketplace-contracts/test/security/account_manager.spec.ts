// SPDX-License-Identifier: MIT
import { after, describe } from 'mocha'

import CreatorABI from '../../artifacts/creator.json'
import UserABI from '../../artifacts/user.json'
import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import {SECURITY_PREFIX} from "../shared/consts";
import { Signers } from '../shared/signers'
import { setupAccountManager as setup } from '../shared/test-setups/account_manager'

describe(SECURITY_PREFIX + 'Account Manager', () => {
  const USER_CODE_HASH = UserABI.source.hash
  const CREATOR_CODE_HASH = CreatorABI.source.hash

  it('Not admin cannot change creator code hash', async () => {
    const contract = await setup()

    await expect(contract
        .withSigner(Signers.Alice)
        .tx
        .setCreatorCodeHash(USER_CODE_HASH)
    ).to.eventually.be.rejected
  })
  it('Not admin cannot change user code hash', async () => {
    const contract = await setup()

    await expect(contract
        .withSigner(Signers.Alice)
        .tx
        .setUserCodeHash(CREATOR_CODE_HASH)
    ).to.eventually.be.rejected
  })

  after(async () => {
    await ApiSingleton.disconnect()
  })
})
