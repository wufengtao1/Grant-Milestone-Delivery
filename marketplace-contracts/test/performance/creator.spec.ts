// SPDX-License-Identifier: MIT
import BN from "bn.js";

import ArchNFTAbi from "../../artifacts/arch_nft.json";
import Contract from "../../typechain-generated/contracts/creator";
import {Hash} from "../../typechain-generated/types-returns/creator";
import ApiSingleton from "../shared/api_singleton";
import {expect} from "../shared/chai";
import {PERFORMANCE_PREFIX} from "../shared/consts";
import {setupCreator} from "../shared/test-setups/creator";

const CREATE_COLLECTION_MAX_FEE = new BN(16_500_000_000);

describe(PERFORMANCE_PREFIX + 'Creator', function() {
    let contract : Contract;

    beforeEach(async function() {
        contract = await setupCreator();
    })

    after(async function() {
        await ApiSingleton.disconnect();
    })

    it('Should create collection within max fee', async function() {
        await expect(contract.query.createCollection(
            "test",
            "test",
            100,
            "123",
            ArchNFTAbi.source.hash as unknown as Hash
        )).to.have.feeLessThan(CREATE_COLLECTION_MAX_FEE)
    });
});
