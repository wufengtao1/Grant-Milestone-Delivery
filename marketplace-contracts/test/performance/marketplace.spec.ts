// SPDX-License-Identifier: MIT
import BN from "bn.js";
import { describe } from 'mocha'

import PSP34Contract from "../../typechain-generated/contracts/arch_nft";
import Contract from "../../typechain-generated/contracts/marketplace";
import PSP22Contract from "../../typechain-generated/contracts/my_psp22";
import {CurrencyBuilder} from "../../typechain-generated/types-arguments/marketplace";
import ApiSingleton from "../shared/api_singleton";
import {expect} from "../shared/chai";
import {
    PERFORMANCE_PREFIX,
    PRICE,
    PRICE_WITH_FEE,
    TOKEN_ID,
    TOKEN_ID_1,
    TOKEN_ID_2,
    TOKEN_ID_3
} from "../shared/consts";
import {mintAndApprove, mintAndList} from "../shared/marketplace";
import {Signers} from "../shared/signers";
import {setupArchNFT} from "../shared/test-setups/arch_nft";
import {setupMarketplace} from "../shared/test-setups/marketplace";
import {setupPSP22} from "../shared/test-setups/my_psp22";

const LIST_NFT_FOR_SALE_MAX_FEE = new BN(25_000_000_000);
const CANCEL_LISTING_MAX_FEE = new BN(14_000_000_000);
const BUY_NFT_MAX_FEE = new BN(21_000_000_000);
const BUY_BATCH_MAX_FEE = new BN(32_000_000_000);

describe(PERFORMANCE_PREFIX + 'Marketplace', function() {
    let contract: Contract;
    let psp22: PSP22Contract;
    let psp34: PSP34Contract;

    beforeEach(async function() {
        contract = await setupMarketplace()

        psp22 = await setupPSP22()
        psp34 = await setupArchNFT()
    })

    after(async function() {
        await ApiSingleton.disconnect();
    })

    it('Should list NFT for sale within max fee', async function() {
        await mintAndApprove(contract, psp34, TOKEN_ID, Signers.Charlie);

        await expect(contract.query.listNftForSale(
            Signers.Charlie.address,
            psp34.address,
            TOKEN_ID,
            PRICE,
            CurrencyBuilder.Custom(psp22.address)
        )).to.have.feeLessThan(LIST_NFT_FOR_SALE_MAX_FEE)
    });

    it('Should cancel listing within max fee', async function() {
        await mintAndList(contract, psp34, psp22, TOKEN_ID, PRICE);

        await expect(contract.query.cancelListing(
            0,
        )).to.have.feeLessThan(CANCEL_LISTING_MAX_FEE)
    });

    it('Should buy NFT within max fee', async function() {
        await mintAndList(contract, psp34, psp22, TOKEN_ID, PRICE);

        await psp22.withSigner(Signers.Alice).tx.approve(contract.address, PRICE_WITH_FEE);

        await expect(contract.query.buyNft(
            0
        )).to.have.feeLessThan(BUY_NFT_MAX_FEE)
    });

    it('Should buy batch within max fee', async function() {
        await mintAndList(contract, psp34, psp22, TOKEN_ID_1, PRICE);
        await mintAndList(contract, psp34, psp22, TOKEN_ID_2, PRICE);
        await mintAndList(contract, psp34, psp22, TOKEN_ID_3, PRICE);

        await psp22.withSigner(Signers.Alice).tx.approve(contract.address, PRICE_WITH_FEE * 3);

        await expect(contract.query.buyBatch(
            [0, 1, 2]
        )).to.have.feeLessThan(BUY_BATCH_MAX_FEE)
    });
});
