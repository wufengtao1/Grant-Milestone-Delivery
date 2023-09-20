// SPDX-License-Identifier: MIT
import BN from "bn.js";
import chai, { Assertion } from 'chai'
import chaiAsPromised from 'chai-as-promised'

import {calculateFee} from "./fees";

chai.use(chaiAsPromised)

declare global {
  export namespace Chai {
    interface Assertion {
      returnValue: (val: any) => Promise<Assertion>
      deepReturnValue: (val: any) => Promise<Assertion>
      returnNumber: (val: number) => Promise<Assertion>
      feeLessThan: (val: BN) => Promise<Assertion>
    }
  }
}

Assertion.addMethod('returnValue', function (this: any, expected: any) {
  const obj = this._obj

  return new Promise((resolve, reject) => {
    obj
      .then((result: any) => {
        const unwrappedValue = result.value.unwrapRecursively()

        this.assert(
          unwrappedValue === expected,
          'expected #{this} to have a return value #{exp} but got #{act}',
          'expected #{this} to not have a return value #{act}',
          expected,
          unwrappedValue,
        )

        resolve(unwrappedValue)
      })
      .catch((error: any) => {
        reject(error)
      })
  })
})

Assertion.addMethod('deepReturnValue', function (this: any, expected: any) {
  const obj = this._obj

  return new Promise((resolve, reject) => {
    obj
      .then((result: any) => {
        const unwrappedValue = result.value.unwrapRecursively()

        this.assert(
          JSON.stringify(unwrappedValue) === JSON.stringify(expected),
          'expected #{this} to have a return value #{exp} but got #{act}',
          'expected #{this} to not have a return value #{act}',
          JSON.stringify(expected),
          JSON.stringify(unwrappedValue),
        )

        resolve(unwrappedValue)
      })
      .catch((error: any) => {
        reject(error)
      })
  })
})

Assertion.addMethod('returnNumber', function (this: any, expected: any) {
  const obj = this._obj

  return new Promise((resolve, reject) => {
    obj
      .then((result: any) => {
        const unwrappedValue = result.value.unwrapRecursively().toNumber()

        this.assert(
          unwrappedValue === expected,
          'expected #{this} to have a return value #{exp} but got #{act}',
          'expected #{this} to not have a return value #{act}',
          expected,
          unwrappedValue,
        )

        resolve(unwrappedValue)
      })
      .catch((error: any) => {
        reject(error)
      })
  })
})

chai.Assertion.addMethod('feeLessThan', function (expected) {
    const obj = this._obj;

    return obj
        .then(async (result: any) => {
            const unwrappedValue = result.gasConsumed;

            const fee = await calculateFee(unwrappedValue); //ensure calculateFee is defined somewhere

            this.assert(
                fee.lt(expected),
                `expected ${this} to have a fee less than ${expected} but got ${fee}`,
                `expected ${this} to not have a fee less than ${fee}`,
                expected.toString(),
                fee.toString(),
            )
        })
        .catch((error: any) => {
            throw error;
        })
});

export const assert = chai.assert
export const expect = chai.expect
