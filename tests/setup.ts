import { createHookPayload, iHook } from "@transia/hooks-toolkit";
import { Client, SetHook, Transaction, Wallet, encode } from "@transia/xrpl";
import { getFeeEstimateXrp } from "@transia/xrpl/dist/npm/sugar";
import { exec as execWithCallback } from "child_process";
import { readFile as readFileWithCallback } from "fs";
import path from "path";
import util from "util";

const exec = util.promisify(execWithCallback);
const readFile = util.promisify(readFileWithCallback);

interface FaucetSuccessResponse {
  // address: 'rPgAY3v5Zag1QK1xgK2YD76drhTiAd6gCE',
  address: string;
  // secret: 'shPBScW8cqfebyDDGvrqfvJbefWvL',
  secret: string;
  // xrp: 10000,
  xrp: number;
  // hash: 'D5DE850DC1B8235D8F91B9A56AB528EAADB1089050EB8DBF7C4C7C559EF3A152',
  hash: string;
  // code: 'tesSUCCESS'
  code: string;
}

interface FaucetErrorResponse {
  // error: 'you must wait 10 seconds before requesting again'
  error: string;
}

export class Faucet {
  static async getNewAccount(): Promise<
    FaucetSuccessResponse | FaucetErrorResponse
  > {
    return fetch(`https://hooks-testnet-v3.xrpl-labs.com/newcreds`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    }).then((resp) => resp.json());
  }

  static async waitAndGetNewAccount(): Promise<FaucetSuccessResponse> {
    let tries = 0;
    while (tries < 20) {
      const resp = await Faucet.getNewAccount();
      if ("error" in resp) {
        await new Promise((resolve) => setTimeout(resolve, 1000));
        tries++;
        continue;
      }
      return resp;
    }

    throw new Error(`Could not get new account after ${tries} tries`);
  }
}

export class TestUtils {
  static async buildHook(hookName: string): Promise<iHook> {
    await exec("cargo +nightly build --examples --release");
    const hook = createHookPayload(
      0,
      // Add hook code after this
      null,
      `${hookName}namespace`,
      undefined,
      ["Invoke"]
    );
    const wasmDir = path.resolve(
      __dirname,
      `..`,
      `target`,
      `wasm32-unknown-unknown`,
      `release`,
      `examples`
    );
    const wasmInFile = path.resolve(wasmDir, `${hookName}.wasm`);
    const wasmOutFile = path.resolve(wasmDir, `${hookName}-cleaned.wasm`);
    await exec(`hook-cleaner ${wasmInFile} ${wasmOutFile}`);
    const wasm = await readFile(wasmOutFile);
    const wasmHex = wasm.toString(`hex`).toUpperCase();
    hook.CreateCode = wasmHex;

    return hook;
  }

  static async getTransactionFee(
    client: Client,
    transaction: Transaction
  ): Promise<string> {
    const copyTx = JSON.parse(JSON.stringify(transaction));
    copyTx.Fee = `0`;
    copyTx.SigningPubKey = ``;

    const preparedTx = await client.autofill(copyTx);

    const tx_blob = encode(preparedTx);

    const result = await getFeeEstimateXrp(client, tx_blob);

    return result;
  }

  static async submitAndWaitWithRetries(
    client: Client,
    ...params: Parameters<Client["submitAndWait"]>
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

  static async setHook(client: Client, secret: string, hook: iHook) {
    const wallet = Wallet.fromSecret(secret);
    const tx: SetHook = {
      TransactionType: `SetHook`,
      Account: wallet.address,
      Hooks: [{ Hook: hook }],
    };

    const fee = await TestUtils.getTransactionFee(client, tx);
    tx.Fee = fee;

    const submitResponse = await TestUtils.submitAndWaitWithRetries(
      client,
      tx,
      {
        wallet,
        failHard: true,
        autofill: true,
      }
    );

    return submitResponse;
  }

  static deserializeHexStringAsBigInt(hexString: string): bigint {
    const SIGN_BIT_MASK = BigInt(`0x8000000000000000`);

    const maybeSignedNumber = BigInt(
      `${hexString.startsWith(`0x`) ? hexString : `0x${hexString}`}`
    );

    const isNegative = (maybeSignedNumber & SIGN_BIT_MASK) !== 0n;

    if (isNegative) {
      return -(maybeSignedNumber ^ SIGN_BIT_MASK);
    } else {
      return maybeSignedNumber;
    }
  }
}
