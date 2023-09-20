// SPDX-License-Identifier: MIT
import RawAbi from '../../../artifacts/arch_nft.json'
import Constructors from '../../../typechain-generated/constructors/creator'
import Contract from '../../../typechain-generated/contracts/creator'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'
import {gasLimit} from "./shared";

export const ADDITIONAL_INFO = JSON.stringify({
  image: 'ipfs://arch-nft.com/1.png',
  name: 'Arch NFT Pilot Collection',
  description: 'Arch NFT Pilot Collection',
})
export const COLLECTION_URI = 'ipfs://arch-nft.com/'
export const COLLECTION_NAME = 'Arch NFT Pilot Collection'
export const COLLECTION_ROYALTY = 100
export const COLLECTION_CODE_HASH = RawAbi.source.hash

export async function setupCreator(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()

  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new(defaultSigner.address, {gasLimit: gasLimit(310000000, 17000)})

  return new Contract(address, defaultSigner, api)
}
