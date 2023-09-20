// SPDX-License-Identifier: MIT
import {KeyringPair} from "@polkadot/keyring/types";
import { describe } from 'mocha'

import Contract from "../../typechain-generated/contracts/my_admin_access";
import ApiSingleton from "../shared/api_singleton";
import {expect} from "../shared/chai";
import {SECURITY_PREFIX} from "../shared/consts";
import {Signers} from "../shared/signers";
import {setupAdminAccess} from "../shared/test-setups/admin_access";

describe(SECURITY_PREFIX + 'AdminAccess', function () {
    let adminAccess: Contract

    let alice: KeyringPair, bob: KeyringPair

    beforeEach(async function () {
        adminAccess = await setupAdminAccess()
        alice = Signers.Alice
        bob = Signers.Bob
    })

    after(async function () {
        await ApiSingleton.disconnect()
    })

    it('Cannot remove admin if not admin', async function () {
        await expect(adminAccess.tx.removeAdmin(bob.address)).to.eventually.be.rejected
    })

    it('Cannot add admin if not admin', async function () {
        await expect(adminAccess.withSigner(alice).tx.addAdmin(bob.address)).to.eventually.be.rejected
    })

    it('Cannot remove admin if not admin', async function () {
        await expect(adminAccess.withSigner(alice).tx.removeAdmin(bob.address)).to.eventually.be.rejected
    })
})