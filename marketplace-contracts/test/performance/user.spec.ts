// SPDX-License-Identifier: MIT
import BN from "bn.js";
import { describe } from 'mocha'

import Contract from "../../typechain-generated/contracts/user";
import ApiSingleton from "../shared/api_singleton";
import {expect} from "../shared/chai";
import {PERFORMANCE_PREFIX} from "../shared/consts";
import {setupUser} from "../shared/test-setups/user";

const SET_USER_DATA_MAX_FEE = new BN(3_800_000_000);

describe(PERFORMANCE_PREFIX + 'Creator', function() {
    let contract : Contract;

    beforeEach(async function() {
        contract = await setupUser();
    })

    after(async function() {
        await ApiSingleton.disconnect();
    })

    it('Should set user data under max fee', async function() {
        await expect(contract.query.setUserData({
            nick: "@nick",
            avatar: {
                id: {
                    u8: 0
                },
                uri: "https://example.com/avatar.png",
                contractAddress: "0x000000"
            },
            additionInfo: "Some info"
        })).to.have.feeLessThan(SET_USER_DATA_MAX_FEE)
    });
});
