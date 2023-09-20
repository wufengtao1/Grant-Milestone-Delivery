// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/my_admin_access'
import Contract from '../../../typechain-generated/contracts/my_admin_access'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'

export async function setupAdminAccess(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()
  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)
  const { address } = await constructors.new()

  return new Contract(address, defaultSigner, api)
}
