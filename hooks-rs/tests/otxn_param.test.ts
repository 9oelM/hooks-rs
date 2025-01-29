// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";
import {
  iHookParamEntry,
  iHookParamName,
  iHookParamValue,
} from "@transia/hooks-toolkit";

const HOOK_NAME = "oxtn_param";

describe("oxtn_param.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    //
    alice = Wallet.fromSecret(`sn1YbwYB2eurfpDkLiTgzUVFBgV4f`);
    // rJCr4EiJ6fAeo6Gxn6b6UPqJaLqRBvh6mg
    bob = Wallet.fromSecret(`ssTGwPho3ND79Mv9miY868LqAqNpG`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "accepts with the value of the hook parameter",
    async () => {
      const OTXN_PARAM_VALUE = `abcdefg123`;

      const tx: Invoke & Transaction = {
        TransactionType: "Invoke",
        Account: bob.classicAddress,
        Destination: alice.classicAddress,
        HookParameters: [
          new iHookParamEntry(
            new iHookParamName("param test"),
            new iHookParamValue(OTXN_PARAM_VALUE),
          ).toXrpl(),
        ],
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

      expect(Number(HookReturnCode)).toBe(0);
      expect(Buffer.from(HookReturnString, "hex").toString()).toMatch(
        OTXN_PARAM_VALUE,
      );
    },
    3 * 60_000,
  );
});
