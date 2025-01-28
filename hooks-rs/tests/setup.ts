import { createHookPayload, iHook } from "@transia/hooks-toolkit";
import {
  Client,
  encode,
  SetHook,
  Transaction,
  TxResponse,
  Wallet,
  XrplError,
} from "@transia/xrpl";
import { getFeeEstimateXrp } from "@transia/xrpl/dist/npm/sugar";
import { exec as execWithCallback } from "child_process";
import { readFile as readFileWithCallback } from "fs";
import path from "path";
import util from "util";

const exec = util.promisify(execWithCallback);
const readFile = util.promisify(readFileWithCallback);

interface FaucetSuccessResponse {
  account: {
    //     "xAddress": "XV5CC9AbwcsBYScgsjxWpe5VMooGZ8n8NMmaNuhUbqHPozq",
    xAddress: string;
    //     "secret": "snTePLtQua9MaLMsRxWuKMQVJV4XP",
    secret: string;
    //     "classicAddress": "rPSTDHkr2n9Fq7jza5Ei1nCoSMVanfLXpV",
    classicAddress: string;
    // address: 'rPgAY3v5Zag1QK1xgK2YD76drhTiAd6gCE',
    address: string;
  };
  amount: number;
  balance: number;
  trace: {
    //     "hash": "236A497826E877596EED24A9E4A59F4E47196DAB09162FA027DFF3A7487E8CD2",
    hash: string;
    //     "code": "tesSUCCESS"
    code: string;
  };
}

interface FaucetErrorResponse {
  // error: 'you must wait 10 seconds before requesting again'
  error: string;
}

export class Faucet {
  static async getNewAccount(): Promise<
    FaucetSuccessResponse | FaucetErrorResponse
  > {
    return fetch(`https://xahau-test.net/accounts`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    }).then((resp) => resp.json());
  }

  static async waitAndGetNewAccount(): Promise<FaucetSuccessResponse> {
    let tries = 0;
    let resp: Awaited<ReturnType<typeof Faucet.getNewAccount>> | undefined;
    while (tries < 20) {
      try {
        resp = await Faucet.getNewAccount();
        // ignore errors
      } catch (e) {}

      if (resp && "error" in resp && resp.error.includes(`you must wait`)) {
        await new Promise((resolve) => setTimeout(resolve, 5000));
        tries++;
        continue;
      } else if (resp && "account" in resp) {
        return resp;
      } else {
        throw new Error(`Unknown error: ${JSON.stringify(resp)}`);
      }
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
    const debugDir = path.resolve(__dirname, `..`, `target`);
    const wasmInFile = path.resolve(wasmDir, `${hookName}.wasm`);
    const wasmOutFlattened = path.resolve(
      wasmDir,
      `${hookName}-flattened.wasm`
    );
    // Maximum allowable depth of blocks reached is 16 levels in hooks GuardCheck.
    // Otherwise, the node will not validate the SetHook transaction.
    // Therefore, flatten it using wasm-opt.
    await exec(
      `wasm-opt ${wasmInFile} --flatten --rereloop -Oz -Oz -o ${wasmOutFlattened}`
    );
    const wasmOutCleaned = path.resolve(wasmDir, `${hookName}-cleaned.wasm`);
    const hookCleanerOut = await exec(
      `hook-cleaner ${wasmOutFlattened} ${wasmOutCleaned}`
    );
    console.log(JSON.stringify(hookCleanerOut, null, 2));
    await Promise.all([
      exec(
        `wasm2wat ${wasmInFile} -o ${path.resolve(debugDir, `${hookName}.wat`)}`
      ),
      exec(
        `wasm2wat ${wasmOutCleaned} -o ${path.resolve(
          debugDir,
          `${hookName}-cleaned.wat`
        )}`
      ),
      exec(
        `wasm2wat ${wasmOutFlattened} -o ${path.resolve(
          debugDir,
          `${hookName}-flattened.wat`
        )}`
      ),
    ]);
    try {
      const guardCheckerOut = await exec(`guard_checker ${wasmOutCleaned}`);
      console.log(JSON.stringify(guardCheckerOut, null, 2));
    } catch (e) {
      console.error(e);

      throw new Error(`Guard checker failed`);
    }
    const wasm = await readFile(wasmOutCleaned);
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

  static async waitForMaybeNonExistentTx(
    client: Client,
    txHash: string
  ): Promise<boolean> {
    let validated = false;
    let tries = 0;
    while (!validated && tries < 20) {
      try {
        const txResponse: TxResponse | boolean = await client.request({
          command: "tx",
          transaction: txHash,
        });
        if (typeof txResponse !== `boolean` && txResponse?.result?.validated) {
          return true;
        }
        await new Promise((resolve) => setTimeout(resolve, 1000));
        tries++;
      } catch (error) {
        // error is of an unknown type and hence we assert type to extract the value we need.
        // eslint-disable-next-line @typescript-eslint/consistent-type-assertions,@typescript-eslint/no-unsafe-member-access -- ^
        const message = ((error as XrplError)?.data as { error: string })
          ?.error as string;
        if (message === "txnNotFound") {
          await new Promise((resolve) => setTimeout(resolve, 1000));
          tries++;
          continue;
        }

        throw new Error(`Failed to get transaction: ${JSON.stringify(error)}`);
      }
    }

    return false;
  }

  static async afterPromise<T>(
    promise: Promise<T>,
    promiseCb: (result: T) => void
  ): Promise<T> {
    const result = await promise;
    promiseCb(result);
    return result;
  }
}
