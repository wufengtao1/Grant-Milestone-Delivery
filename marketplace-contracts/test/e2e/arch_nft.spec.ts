// SPDX-License-Identifier: MIT
import { after, describe } from 'mocha'

import Constructors from '../../typechain-generated/constructors/arch_nft'
import { IdBuilder } from '../../typechain-generated/types-arguments/arch_nft'
import ApiSingleton from '../shared/api_singleton'
import { expect } from '../shared/chai'
import { E2E_PREFIX } from '../shared/consts'
import { Signers } from '../shared/signers'
import { setupArchNFT as setup } from '../shared/test-setups/arch_nft'
import { ADDITIONAL_INFO, COLLECTION_NAME, COLLECTION_URI } from '../shared/test-setups/creator'

describe(E2E_PREFIX + 'Arch NFT', () => {
  it('Metadata works', async () => {
    const contract = await setup()

    await expect(contract.query.collectionRoyalty()).to.have.returnValue(100)
    await expect(contract.query.collectionName()).to.have.returnValue(COLLECTION_NAME)
    await expect(contract.query.collectionUri()).to.have.returnValue(COLLECTION_URI)
    await expect(contract.query.collectionAdditionalInfo()).to.have.returnValue(ADDITIONAL_INFO)
  })

  it('Cannot set royalty more than 10000', async () => {
    const api = await ApiSingleton.getInstance()

    const defaultSigner = Signers.defaultSigner

    const constructors = new Constructors(api, defaultSigner)

    await expect(constructors.new(10001, COLLECTION_NAME, COLLECTION_URI, ADDITIONAL_INFO)).to.eventually.be.rejected
  })

  it('Royalty less or equal than 10000 works', async () => {
    const api = await ApiSingleton.getInstance()

    const defaultSigner = Signers.defaultSigner

    const constructors = new Constructors(api, defaultSigner)

    await expect(constructors.new(10000, COLLECTION_NAME, COLLECTION_URI, ADDITIONAL_INFO)).to.eventually.be.fulfilled
    await expect(constructors.new(1, COLLECTION_NAME, COLLECTION_URI, ADDITIONAL_INFO)).to.eventually.be.fulfilled
  })

  it('Can mint NFT', async () => {
    const contract = await setup()

    await contract.tx.mint(Signers.Alice.address, IdBuilder.U8(1))

    await expect(contract.query.ownerOf(IdBuilder.U8(1))).to.have.returnValue(Signers.Alice.address)
  })

  it('Can transfer NFT', async () => {
    const contract = await setup()

    const alice = Signers.Alice
    const bob = Signers.Bob

    await contract.tx.mint(alice.address, IdBuilder.U8(1))
    await contract.withSigner(alice).tx.transfer(bob.address, IdBuilder.U8(1), [])

    await expect(contract.query.ownerOf(IdBuilder.U8(1))).to.have.returnValue(bob.address)
  })

  it('Can burn NFT', async () => {
    const contract = await setup()

    const alice = Signers.Alice

    await contract.tx.mint(alice.address, IdBuilder.U8(1))
    await contract.tx.burn(alice.address, IdBuilder.U8(1))

    await expect(contract.query.ownerOf(IdBuilder.U8(1))).to.have.returnValue(null)
  })

  it('Can set collection name', async () => {
    const contract = await setup()

    await contract.tx.setCollectionName('New name')

    await expect(contract.query.collectionName()).to.have.returnValue('New name')
  })

  it('Can set collection uri', async () => {
    const contract = await setup()

    await contract.tx.setCollectionUri('New uri')

    await expect(contract.query.collectionUri()).to.have.returnValue('New uri')
  })

  it('Can set collection additional info', async () => {
    const contract = await setup()

    await contract.tx.setCollectionAdditionalInfo('New info')

    await expect(contract.query.collectionAdditionalInfo()).to.have.returnValue('New info')
  })

  after(async () => {
    await ApiSingleton.disconnect()
  })
})
