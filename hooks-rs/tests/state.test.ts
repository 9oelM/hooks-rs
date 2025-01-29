// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";
import { padHexString, StateUtility } from "@transia/hooks-toolkit";

const HOOK_NAME = "state";

describe("state.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // rHZBYi9RLCsM6oW9L6A8vMWSBfoiagRH6m
    alice = Wallet.fromSecret(`ssDhYg2KDrCh7FHWvkuVr2PXd5kfw`);
    // rhoh3g6As6MdR7nrbnyiPBig8mriDJDdXN
    bob = Wallet.fromSecret(`ssiRvVtZMhr989j2BMVqmGkYPP1cC`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "sets a state value with a specific key",
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

      // Hook always returns uppercase hex string
      const stateKey = padHexString(
        Buffer.from(`hello world key`).toString("hex").toUpperCase(),
      );
      // Hook always returns uppercase hex string
      const expectedHookStateData = Buffer.from(`hello world val`)
        .toString("hex")
        .toUpperCase();

      const actualState = await StateUtility.getHookState(
        client,
        alice.classicAddress,
        stateKey,
        `${HOOK_NAME}namespace`,
      );

      expect(actualState.HookStateData).toBe(expectedHookStateData);
      expect(actualState.HookStateKey).toBe(stateKey);

      // safe type: we checked everything
      const [hookExecution] = meta.HookExecutions as [HookExecution];

      const { HookReturnCode, HookReturnString } = hookExecution.HookExecution;

      expect(Number(HookReturnCode)).toBe(0);
      // Hook state data is also returned as a parameter to 'accept' function
      expect(HookReturnString).toBe(expectedHookStateData);
    },
    3 * 60_000,
  );
});
