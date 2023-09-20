// SPDX-License-Identifier: MIT
import { setupAccountManager } from '../test/shared/test-setups/account_manager'
import { setupMarketplace } from '../test/shared/test-setups/marketplace'
import { setupPSP22 } from '../test/shared/test-setups/my_psp22'

export async function base(): Promise<void> {
  const accountManager = await setupAccountManager()

  const psp22 = await setupPSP22()

  const marketplace = await setupMarketplace()

  /* eslint-disable no-console */
  console.log({
    AccountManager: accountManager.address,
    PSP22: psp22.address,
    Marketplace: marketplace.address,
  })
}
