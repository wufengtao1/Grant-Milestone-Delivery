// SPDX-License-Identifier: MIT
import CreatorAbi from '../../../artifacts/creator.json'
import UserAbi from '../../../artifacts/user.json'
import Constructors from '../../../typechain-generated/constructors/account_manager'
import Contract from '../../../typechain-generated/contracts/account_manager'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'
import {gasLimit} from "./shared";

export async function setupAccountManager(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()
  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new(UserAbi.source.hash, CreatorAbi.source.hash, {
    gasLimit: gasLimit(2500000000, 36000),
  })

  return new Contract(address, defaultSigner, api)
}
