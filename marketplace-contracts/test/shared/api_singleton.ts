// SPDX-License-Identifier: MIT
import { ApiPromise } from '@polkadot/api'

export default class ApiSingleton {
  private static instance: ApiPromise | undefined

  public static async getInstance(): Promise<ApiPromise> {
    if (!ApiSingleton.instance) {
      ApiSingleton.instance = await ApiPromise.create()
    }

    return ApiSingleton.instance
  }

  public static getInstanceSync(): ApiPromise {
    if (!ApiSingleton.instance) {
      throw new Error('ApiSingleton is not initialized')
    }

    return ApiSingleton.instance
  }

  public static async disconnect(): Promise<void> {
    if (ApiSingleton.instance) {
      await ApiSingleton.instance.disconnect()
    }

    ApiSingleton.instance = undefined
  }

  public static async initWithApi(api: ApiPromise): Promise<void> {
    ApiSingleton.instance = api
  }
}
