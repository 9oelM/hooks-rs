// xrpl
import {
  Client,
  decodeAccountID,
  Invoke,
  Transaction,
  Wallet,
} from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";
import { iHook, padHexString, StateUtility } from "@transia/hooks-toolkit";

const HOOK_NAME = "state_basic";

describe("state_basic.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;
  let hook: iHook;

  beforeAll(async () => {
    hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
  }, 3 * 60_000);

  beforeEach(async () => {
    // rfv8LQPv4a7DbmvjGBqW2TSEt864wJ5Vgf
    alice = Wallet.fromSecret(`snoUzNNV2m3K9dW7oJEebmRSkHsHX`);
    // r4gLsVufLvPsEUGkAPEeateoLLmeT7esPn
    bob = Wallet.fromSecret(`snbFSGYKuzrTjLtwmQtzd32PtcmJ4`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "should set count as 2 in the hook's state for the two state keys from hook account and oxtn account",
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

      if (txResponse.result.meta.TransactionResult !== "tesSUCCESS") {
        console.error(JSON.stringify(txResponse, null, 2));

        throw new Error(`Transaction failed`);
      }

      for (const address of [alice.classicAddress, bob.classicAddress]) {
        // Hook always returns uppercase hex string
        const addressAsStateKey = padHexString(
          decodeAccountID(address).toString("hex").toUpperCase(),
        );
        // Hook always returns uppercase hex string
        const actualState = await StateUtility.getHookState(
          client,
          alice.classicAddress,
          addressAsStateKey,
          `${HOOK_NAME}namespace`,
        );

        expect(
          TestUtils.deserializeHexStringAsBigInt(actualState.HookStateData),
        ).toBe(2n);
        expect(actualState.HookStateKey).toBe(addressAsStateKey);
      }

      // safe type: we checked everything
      const [hookExecution] = meta.HookExecutions as [HookExecution];

      const { HookReturnCode, HookReturnString } = hookExecution.HookExecution;

      expect(Number(HookReturnCode)).toBe(0);
      // Hook state data is also returned as a parameter to 'accept' function
      expect(TestUtils.deserializeHexStringAsBigInt(HookReturnString)).toBe(2n);
    },
    3 * 60_000,
  );
});
