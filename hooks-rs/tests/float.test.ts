// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { Faucet, TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";

const HOOK_NAME = "float";

describe("float.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    let [{ account: { secret: secret0 } }, { account: { secret: secret1 } }] = await Promise.all([
      Faucet.waitAndGetNewAccount(),
      Faucet.waitAndGetNewAccount(),
    ]);
    alice = Wallet.fromSecret(secret0);
    bob = Wallet.fromSecret(secret1);
    await new Promise((resolve) => setTimeout(resolve, 5000));
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    `should correctly run float operations and end with accept("", 0)`,
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

      const { meta } = txResponse.result;
      if (!(meta.HookExecutions && meta.HookExecutions.length > 0)) {
        throw new Error(`Hook execution data is empty`);
      }

      if (meta.HookExecutions.length > 1) {
        throw new Error(`Hook execution happened more than once`);
      }

      // safe type: we checked everything
      const [hookExecution] = meta.HookExecutions as [HookExecution];

      const { HookReturnCode } = hookExecution.HookExecution;

      expect(
        TestUtils.deserializeHexStringAsBigInt(HookReturnCode.toString()),
      ).toBe(0n);
    },
    3 * 60_000,
  );
});
