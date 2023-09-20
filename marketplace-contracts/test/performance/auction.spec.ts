// SPDX-License-Identifier: MIT
import BN from "bn.js";

import PSP34Contract from "../../typechain-generated/contracts/arch_nft";
import Contract from "../../typechain-generated/contracts/mock_auction";
import PSP22Contract from "../../typechain-generated/contracts/my_psp22";
import {CurrencyBuilder} from "../../typechain-generated/types-arguments/mock_auction";
import ApiSingleton from "../shared/api_singleton";
import {expect} from "../shared/chai";
import {MIN_BID_STEP, PERFORMANCE_PREFIX, PRICE, PRICE_WITH_FEE, TOKEN_ID} from "../shared/consts";
import {genTime, mintAndApprove, mintAndListAuction} from "../shared/mock_auction";
import {Signers} from "../shared/signers";
import {setupArchNFT} from "../shared/test-setups/arch_nft";
import {setupMockAuction} from "../shared/test-setups/mock_auction";
import {setupPSP22} from "../shared/test-setups/my_psp22";

const LIST_NFT_FOR_AUCTION_MAX_FEE = new BN(26_000_000_000);
const START_AUCTION_MAX_FEE = new BN(4_500_000_000);
const CANCEL_AUCTION_MAX_FEE = new BN(13_500_000_000);
const BID_NFT_MAX_FEE = new BN(3_400_000_000);
const CLAIM_NFT_MAX_FEE = new BN(35_000_000_000);

describe(PERFORMANCE_PREFIX + 'Auction', function() {
    let contract: Contract;
    let psp22: PSP22Contract;
    let psp34: PSP34Contract;

    beforeEach(async function() {
        contract = await setupMockAuction()

        psp22 = await setupPSP22()
        psp34 = await setupArchNFT()

    })

    after(async function() {
        await ApiSingleton.disconnect();
    })

    it('Should list NFT for auction within max fee', async function() {
        await mintAndApprove(contract, psp34, TOKEN_ID, Signers.Charlie);

        const currentTime = (await contract.query.timestamp()).value.unwrapRecursively()

        const time = genTime(currentTime, 10, 100)

        await expect(contract.query.listNftForAuction(
            {
                creator: Signers.Charlie.address,
                collection: psp34.address,
                tokenId: TOKEN_ID,
                startPrice: PRICE,
                minBidStep: MIN_BID_STEP,
                currency: CurrencyBuilder.Custom(psp22.address),
                startTime: time.START_TIME,
                endTime: time.END_TIME
            },
        )).to.have.feeLessThan(LIST_NFT_FOR_AUCTION_MAX_FEE)
    });

    it('Should start auction within max fee', async function() {
        await mintAndListAuction(contract, psp34, psp22, TOKEN_ID, PRICE, MIN_BID_STEP);

        await contract.tx.addTimestamp(3001)

        await expect(contract.query.startAuction(
            0
        )).to.have.feeLessThan(START_AUCTION_MAX_FEE)
    });

    it('Should cancel auction within max fee', async function() {
        await mintAndListAuction(contract, psp34, psp22, TOKEN_ID, PRICE, MIN_BID_STEP);

        await expect(contract.query.cancelAuction(
            0
        )).to.have.feeLessThan(CANCEL_AUCTION_MAX_FEE)
    });

    it('Should bid on NFT within max fee', async function() {
        await mintAndListAuction(contract, psp34, psp22, TOKEN_ID, PRICE, MIN_BID_STEP)

        await contract.tx.addTimestamp(3001)

        await contract.tx.startAuction(0)

        await contract.tx.addTimestamp(5001)

        await psp22.withSigner(Signers.Alice).tx.approve(contract.address, 2 * PRICE_WITH_FEE)

        await expect(contract.withSigner(Signers.Alice).query.bidNft(
            0,
            2 * PRICE,
        )).to.have.feeLessThan(BID_NFT_MAX_FEE)
    });

    it('Should claim NFT within max fee', async function() {
        await mintAndListAuction(contract, psp34, psp22, TOKEN_ID, PRICE, MIN_BID_STEP, false, 100, 20);

        await contract.tx.addTimestamp(50);

        await contract.tx.startAuction(0);

        await psp22.withSigner(Signers.Alice).tx.approve(contract.address, 2 * PRICE_WITH_FEE);

        await contract.withSigner(Signers.Alice).tx.bidNft(
            0,
            2 * PRICE,
        )

        await contract.tx.addTimestamp(100);

        await expect(contract.withSigner(Signers.Alice).query.claimNft(
            0,
        )).to.have.feeLessThan(CLAIM_NFT_MAX_FEE)
    });
});
