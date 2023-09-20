import {WeightV2} from "@polkadot/types/interfaces";
import BN from "bn.js";

import ApiSingleton from "../api_singleton";

export function gasLimit(refTime: number, proofSize: number): WeightV2 {
    const api = ApiSingleton.getInstanceSync()
    const registry = api.registry

    return {
        refTime: registry.createType("Compact<u64>", new BN(refTime)),
        proofSize: registry.createType("Compact<u64>", new BN(proofSize)),
    } as unknown as WeightV2
}