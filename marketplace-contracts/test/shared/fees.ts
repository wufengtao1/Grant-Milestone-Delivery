// SPDX-License-Identifier: MIT
import {WeightV2} from "@polkadot/types/interfaces";
import BN from "bn.js";

import ApiSingleton from "./api_singleton";

export async function calculateFee(weight: WeightV2): Promise<BN> {
    const api = await ApiSingleton.getInstance()

    const fee = await api.call.transactionPaymentApi.queryWeightToFee(weight)

    return fee as unknown as BN
}
