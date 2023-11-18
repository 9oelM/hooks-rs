import * as path from "https://deno.land/std@0.207.0/path/mod.ts";
// @deno-types="./types/command-exists.d.ts"
import commandExists from "npm:command-exists"
import * as xrpl from "npm:@transia/xrpl"
import { TypedObjectKeys } from "./types/utils.ts";
import { SimplifiedHooksToolkit } from "./simplified-hooks-toolkit.ts";
import { getFeeEstimateXrp } from "npm:@transia/xrpl/dist/npm/sugar/index.js";
import { Hex } from "./hex.ts";
import { HookPayload } from "./types/hooks.ts";

export class HooksRsSetup {
  // check inside cargo workplace
  async installCargo() {

  }

  static async checkPrerequisitesInstalled() {
    const prerequisitesInstallationStatus = {
      cargo: false,
      ['wasm-opt']: false,
      ['hook-cleaner']: false,
      ['wasm2wat']: false,
      ['guard_checker']: false,
    }

    for (const prerequisite of TypedObjectKeys(prerequisitesInstallationStatus)) {
      try {
        await commandExists(prerequisite);
        prerequisitesInstallationStatus[prerequisite] = true;
      } catch {}
    }

    return prerequisitesInstallationStatus
  }
}

export class HooksRsManager {
  private static handleOutput(output: Deno.CommandOutput) {
    if (output.success) {
      console.log(output.stdout);
    } else {
      console.error(output.stderr);
    }
  }

  static async buildHook(hookName: string): Promise<HookPayload> {
    const cargoBuildOutput = await new Deno.Command(`cargo`, { args: [
      "+nightly",
      "build",
      "--release",
    ] }).output();
    this.handleOutput(cargoBuildOutput);
    const hook = SimplifiedHooksToolkit.createHookPayload(
      0,
      `${hookName}namespace`,
      undefined,
    );
    const wasmDir = path.join(
      Deno.cwd(),
      `target`,
      `wasm32-unknown-unknown`,
      `release`,
    );
    const debugDir = path.join(Deno.cwd(), "target");
    const wasmInFile = path.join(wasmDir, `${hookName}.wasm`);
    const wasmOutFlattened = path.join(
      wasmDir,
      `${hookName}-flattened.wasm`,
    );
    // Maximum allowable depth of blocks reached is 16 levels in hooks GuardCheck.
    // Otherwise, the node will not validate the SetHook transaction.
    // Therefore, flatten it using wasm-opt.
    const wasmOptOutput = await new Deno.Command(
      `wasm-opt`,
      {
        args: [
          wasmInFile,
          `--flatten`,
          `--rereloop`,
          `-Oz`,
          `-Oz`,
          `-o`,
          wasmOutFlattened,
        ],
      }
    ).output();
    this.handleOutput(wasmOptOutput);
    const wasmOutCleaned = path.join(wasmDir, `${hookName}-cleaned.wasm`);
    const hookCleanerOut = await new Deno.Command(
      `hook-cleaner`,
      {
        args: [
          wasmOutFlattened,
          wasmOutCleaned,
        ],
      }
    ).output();
    console.log(JSON.stringify(hookCleanerOut, null, 2));
    await Promise.all([
      new Deno.Command(
        `wasm2wat`,
        {
          args: [
            wasmInFile,
            `-o`,
            path.join(
              debugDir,
              `${hookName}.wat`,
            ),
          ],
        }
      ).output(),
      new Deno.Command(
        `wasm2wat`,
        {
          args: [
            wasmOutCleaned,
            `-o`,
            path.join(
              debugDir,
              `${hookName}-cleaned.wat`,
            ),
          ]
        }
      ).output(),
      new Deno.Command(
        `wasm2wat`,
        {
          // ${wasmOutFlattened} -o ${path.join(
          //   debugDir,
          //   `${hookName}-flattened.wat`,
          // )}
          args: [
            wasmOutFlattened,
            `-o`,
            path.join(
              debugDir,
              `${hookName}-flattened.wat`,
            ),
          ]
        }
      ).output(),
    ]);
    const guardCheckerOut = await new Deno.Command(`guard_checker`, {
      args: [
        wasmOutCleaned
      ]
    }).output();
    this.handleOutput(guardCheckerOut);

    const wasm = await Deno.readFile(wasmOutCleaned);
    const wasmHex = Hex.uint8ArrayToHexString(wasm).toUpperCase();
    hook.CreateCode = wasmHex;

    return hook;
  }

  static async getTransactionFee(
    client: xrpl.Client,
    transaction: xrpl.Transaction,
  ): Promise<string> {
    const copyTx = JSON.parse(JSON.stringify(transaction));
    copyTx.Fee = `0`;
    copyTx.SigningPubKey = ``;

    const preparedTx = await client.autofill(copyTx);

    const tx_blob = xrpl.encode(preparedTx);

    const result = await getFeeEstimateXrp(client, tx_blob);

    return result;
  }

  static async submitAndWaitWithRetries(
    client: xrpl.Client,
    ...params: Parameters<xrpl.Client["submitAndWait"]>
  ) {
    let tries = 0;
    while (tries < 3) {
      try {
        const result = await client.submitAndWait(...params);
        return result;
      } catch (e) {
        console.error(`${e} - retrying...`);
        await new Promise((resolve) => setTimeout(resolve, 1000));
        tries++;
        continue;
      }
    }

    throw new Error(`Could not submit transaction after ${tries} tries`);
  }

  static async setHook(client: xrpl.Client, secret: string, hook: HookPayload) {
    const wallet = xrpl.Wallet.fromSecret(secret);
    const tx: xrpl.SetHook = {
      TransactionType: `SetHook`,
      Account: wallet.address,
      Hooks: [{ Hook: hook }],
    };

    const fee = await this.getTransactionFee(client, tx);
    tx.Fee = fee;

    const submitResponse = await this.submitAndWaitWithRetries(
      client,
      tx,
      {
        wallet,
        failHard: true,
        autofill: true,
      },
    );

    return submitResponse;
  }
}