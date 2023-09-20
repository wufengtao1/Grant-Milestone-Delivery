// SPDX-License-Identifier: MIT
import ApiSingleton from "../test/shared/api_singleton";
import {base} from "./base";

async function local() {
    await base()

    await ApiSingleton.disconnect()
}

local().then(() => process.exit(0)).catch(error => {
    /* eslint-disable no-console */
    console.error(error);
    process.exit(1);
});