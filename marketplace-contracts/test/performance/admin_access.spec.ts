// SPDX-License-Identifier: MIT
import BN from 'bn.js'

import Contract from '../../typechain-generated/contracts/my_admin_access'
import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import { PERFORMANCE_PREFIX } from '../shared/consts'
import { Signers } from '../shared/signers'
import { setupAdminAccess } from '../shared/test-setups/admin_access'

const ADD_ADMIN_MAX_FEE = new BN(3_000_000_000)
const REMOVE_ADMIN_MAX_FEE = new BN(3_000_000_000)

describe(PERFORMANCE_PREFIX + 'AdminAccess contract', function () {
  let contract: Contract

  beforeEach(async function () {
    contract = await setupAdminAccess()
  })

  after(async function () {
    await ApiSingleton.disconnect()
  })

  it('Add admin', async function () {
    await expect(contract.query.addAdmin(Signers.Alice.address)).to.have.feeLessThan(ADD_ADMIN_MAX_FEE)
  })

  it('Remove admin', async function () {
    await contract.query.addAdmin(Signers.Alice.address)
    await expect(contract.query.removeAdmin(Signers.Alice.address)).to.have.feeLessThan(REMOVE_ADMIN_MAX_FEE)
  })
})
