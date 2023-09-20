// SPDX-License-Identifier: MIT
import Constructors from '../../../typechain-generated/constructors/my_psp22'
import Contract from '../../../typechain-generated/contracts/my_psp22'
import ApiSingleton from '../api_singleton'
import { Signers } from '../signers'

export const INITIAL_BALANCE = 1_000_000_000

export async function setupPSP22(): Promise<Contract> {
  const api = await ApiSingleton.getInstance()
  const defaultSigner = Signers.defaultSigner

  const constructors = new Constructors(api, defaultSigner)

  const { address } = await constructors.new()

  const contract = new Contract(address, defaultSigner, api)

  await contract.tx.mint(defaultSigner.address, INITIAL_BALANCE)
  await contract.tx.mint(Signers.Alice.address, INITIAL_BALANCE)
  await contract.tx.mint(Signers.Bob.address, INITIAL_BALANCE)

  return contract
}
