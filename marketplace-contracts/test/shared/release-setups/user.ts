// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/user'
import Contract from '../../../typechain-generated/contracts/user'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'
import {gasLimit} from "./shared";

export async function setupUser(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()

  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new(defaultSigner.address, {gasLimit: gasLimit(310000000, 17000)})

  return new Contract(address, defaultSigner, api)
}
