// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { ExecutionUtility } from "@transia/hooks-toolkit";
import { TestUtils } from "./setup";

const HOOK_NAME = "fee_base";

describe("fee_base.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // Because Faucet only allows one account to be created every 60 seconds,
    // we will use the following accounts for testing. Change the secrets when
    // running out of funds.
    // rskv5aj9YUE5zkFkzCMQwRW5HVvbcFme61
    alice = Wallet.fromSecret(`shoFfDJhGCbfpppg25QCyPh2s9DoX`);
    // r4y53NaQfLrK9x6oWZ7Lnzct1vejFfP9pn
    bob = Wallet.fromSecret(`ssoiVBiqwKb9JpdRNP9eGd58WkyK8`);

    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "gives base fee",
    async () => {
      const tx: Invoke & Transaction = {
        TransactionType: "Invoke",
        Account: bob.classicAddress,
        Destination: alice.classicAddress,
      };
      const feeResponse = await client.request({
        command: `fee`,
      });
      // Autofilling fee does not work with hooks yet
      const { Fee, ...rest } = await client.autofill(tx);
      const fee = await TestUtils.getTransactionFee(client, rest);
      const txResponse = await TestUtils.submitAndWaitWithRetries(
        client,
        {
          ...tx,
          Fee: fee,
        },
        {
          wallet: bob,
          autofill: true,
        },
      );
      if (!txResponse.result.meta) {
        throw new Error("No meta in tx response");
      }
      if (typeof txResponse.result.meta === "string") {
        throw new Error("Meta is string, not object");
      }

      if (txResponse.result.meta.TransactionResult !== "tesSUCCESS") {
        console.error(JSON.stringify(txResponse, null, 2));

        throw new Error(`Transaction failed`);
      }

      const hookExecutions = await ExecutionUtility.getHookExecutionsFromMeta(
        client,
        txResponse.result.meta,
      );
      if (!hookExecutions.executions[0]) {
        throw new Error(`Hook execution data is empty`);
      }

      expect(hookExecutions.executions[0].HookReturnString).toMatch("");
      expect(
        TestUtils.deserializeHexStringAsBigInt(
          hookExecutions.executions[0].HookReturnCode.toString(),
        ),
      ).toBe(BigInt(feeResponse.result.drops.base_fee));
    },
    3 * 60_000,
  );
});
