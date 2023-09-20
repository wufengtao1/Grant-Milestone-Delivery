// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/user'
import Contract from '../../../typechain-generated/contracts/user'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'

export async function setupUser(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()

  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new(defaultSigner.address)

  return new Contract(address, defaultSigner, api)
}
