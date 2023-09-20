// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/my_psp22'
import Contract from '../../../typechain-generated/contracts/my_psp22'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'
import {gasLimit} from "./shared";

export async function setupPSP22(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()
  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new({gasLimit: gasLimit(140000000, 17000)})

  const contract = new Contract(address, defaultSigner, api)

  return contract
}
