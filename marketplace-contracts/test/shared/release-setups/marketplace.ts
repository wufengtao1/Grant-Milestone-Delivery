// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/marketplace'
import Contract from '../../../typechain-generated/contracts/marketplace'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'
import {gasLimit} from "./shared";

export async function setupMarketplace(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()

  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new(defaultSigner.address, {gasLimit: gasLimit(780000000, 19000)})

  return new Contract(address, defaultSigner, api)
}
