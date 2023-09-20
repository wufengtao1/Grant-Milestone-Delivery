// SPDX-License-Identifier: MIT
import {KeyringPair} from "@polkadot/keyring/types";
import { describe } from 'mocha'

import Contract from "../../typechain-generated/contracts/my_admin_access";
import ApiSingleton from "../shared/api_singleton";
import {expect} from "../shared/chai";
import {E2E_PREFIX} from "../shared/consts";
import {Signers} from "../shared/signers";
import {setupAdminAccess} from "../shared/test-setups/admin_access";

describe(E2E_PREFIX + 'AdminAccess', function () {
    let adminAccess: Contract

    let bob: KeyringPair

    beforeEach(async function () {
        adminAccess = await setupAdminAccess()
        bob = Signers.Bob
    })

    after(async function () {
        await ApiSingleton.disconnect()
    })

    it('Can set admin', async function () {
        await expect(adminAccess.tx.addAdmin(bob.address)).to.eventually.be.fulfilled
    })

    it('Can remove admin', async function () {
        await expect(adminAccess.tx.addAdmin(bob.address)).to.eventually.be.fulfilled

        await expect(adminAccess.tx.removeAdmin(bob.address)).to.eventually.be.fulfilled
    })

    it('Check if admin', async function () {
        await expect(adminAccess.tx.addAdmin(bob.address)).to.eventually.be.fulfilled

        await expect(adminAccess.query.isAdmin(bob.address)).to.have.returnValue(true)
    })
})