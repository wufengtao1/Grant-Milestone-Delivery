// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/account_manager'
import Contract from '../../../typechain-generated/contracts/account_manager'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'
import { setupCreator } from './creator'
import { setupUser } from './user'

export async function setupAccountManager(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()
  const defaultSigner = Signers.defaultSigner

  const creator = await setupCreator()
  const user = await setupUser()

  const constructors = new Constructors(api, defaultSigner)

  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const { address } = await constructors.new(user.contractAbi.json.source.hash, creator.contractAbi.json.source.hash)

  return new Contract(address, defaultSigner, api)
}
