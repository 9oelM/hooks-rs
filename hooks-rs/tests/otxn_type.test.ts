// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";

const HOOK_NAME = "otxn_type";

describe("otxn_type.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // r9CavMR8ZzXA3ARmxyA6PymimGE571aGe4
    alice = Wallet.fromSecret(`shpProjUCuJLev3Uk1xzhzZeLaMBA`);
    // rn5r2h5JaJQTeK5be2ByhpNXimBzAsFSrX
    bob = Wallet.fromSecret(`shc8YTU2AwW68nNutd6kzA73cMmPc`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "accepts with otxn_type",
    async () => {
      const OTXN_TRANSACTION_TYPE = 99n; // ttINVOKE is 99

      const tx: Invoke & Transaction = {
        TransactionType: "Invoke",
        Account: bob.classicAddress,
        Destination: alice.classicAddress,
      };
      // Autofilling fee does not work with hooks yet
      const { Fee, ...rest } = await client.autofill(tx);
      const fee = await TestUtils.getTransactionFee(client, rest);
      const txResponse = await TestUtils.submitAndWaitWithRetries(
        client,
        {
          Fee: fee,
          ...tx,
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

      const { meta } = txResponse.result;
      if (!(meta.HookExecutions && meta.HookExecutions.length > 0)) {
        throw new Error(`Hook execution data is empty`);
      }

      if (meta.HookExecutions.length > 1) {
        throw new Error(`Hook execution happened more than once`);
      }

      if (txResponse.result.meta.TransactionResult !== "tesSUCCESS") {
        console.error(JSON.stringify(txResponse, null, 2));

        throw new Error(`Transaction failed`);
      }

      // safe type: we checked everything
      const [hookExecution] = meta.HookExecutions as [HookExecution];

      const { HookReturnString, HookReturnCode } = hookExecution.HookExecution;

      expect(HookReturnString).toBe("");
      expect(
        TestUtils.deserializeHexStringAsBigInt(HookReturnCode.toString()),
      ).toBe(OTXN_TRANSACTION_TYPE);
    },
    3 * 60_000,
  );
});
