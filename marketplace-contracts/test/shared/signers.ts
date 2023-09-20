// SPDX-License-Identifier: MIT
import { Keyring } from '@polkadot/api'
import { KeyringPair } from '@polkadot/keyring/types'

export class Signers {
  private static keyring: Keyring | undefined
  private static alice: KeyringPair | undefined
  private static bob: KeyringPair | undefined
  private static charlie: KeyringPair | undefined

  private static _keyring(): Keyring {
    if (!this.keyring) {
      this.keyring = new Keyring({ type: 'sr25519' })
    }

    return this.keyring
  }

  public static setDefaultSigner(mnemonic: string): void {
    this.charlie = this._keyring().addFromMnemonic(mnemonic)
  }

  static get Alice(): KeyringPair {
    if (!this.alice) {
      this.alice = this._keyring().addFromUri('//Alice')
    }

    return this.alice
  }

  static get Bob(): KeyringPair {
    if (!this.bob) {
      this.bob = this._keyring().addFromUri('//Bob')
    }

    return this.bob
  }

  static get Charlie(): KeyringPair {
    if (!this.charlie) {
      this.charlie = this._keyring().addFromUri('//Charlie')
    }

    return this.charlie
  }

  static get defaultSigner(): KeyringPair {
    return this.Charlie
  }
}
