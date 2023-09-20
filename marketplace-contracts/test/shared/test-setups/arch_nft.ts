// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/arch_nft'
import Contract from '../../../typechain-generated/contracts/arch_nft'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'

export const ADDITIONAL_INFO = JSON.stringify({
  image: 'ipfs://arch-nft.com/1.png',
  name: 'Arch NFT Pilot Collection',
  description: 'Arch NFT Pilot Collection',
})
export const COLLECTION_URI = 'ipfs://arch-nft.com/'
export const COLLECTION_NAME = 'Arch NFT Pilot Collection'

export async function setupArchNFT(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()

  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new(100, COLLECTION_NAME, COLLECTION_URI, ADDITIONAL_INFO)

  return new Contract(address, defaultSigner, api)
}
