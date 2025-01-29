// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { ExecutionUtility } from "@transia/hooks-toolkit";
import { TestUtils } from "./setup";

const HOOK_NAME = "hooks-rs-template";

describe("accept.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // Replace the secrets with your own
    // by running curl https://xahau-test.net/accounts -XPOST
    //
    // rPytuHMiYpEA49BCiz8378BRjZx6QVBYav
    alice = Wallet.fromSecret(`shNLn8wgcz3fT9t4E4jpLGZ8p3yYh`);
    // rMvs84BS9X48o2C6dFTtVYrPachPjok1CC
    bob = Wallet.fromSecret(`spjC7qpRgz2bxmZeC9YXBjMvYaVkM`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  // afterAll(async () => {
  //   await client.disconnect();
  // }, 10_000);

  it(
    "accepts an incoming txn",
    async () => {
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
      const hookExecutions = await ExecutionUtility.getHookExecutionsFromMeta(
        client,
        txResponse.result.meta,
      );
      if (!hookExecutions.executions[0]) {
        throw new Error(`Hook execution data is empty`);
      }

      expect(hookExecutions.executions[0].HookReturnString).toMatch(
        "accept.rs: Finished.",
      );
    },
    3 * 60_000,
  );
});
