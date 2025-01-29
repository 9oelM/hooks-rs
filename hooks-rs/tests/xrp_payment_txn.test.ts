// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";

const HOOK_NAME = "xrp_payment_txn";

describe("xrp_payment_txn.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    // rDvQ6RxKjFCVPXWJ63MnYJwV2zDj5vg5Vj
    alice = Wallet.fromSecret(`shPxyMAhKBaUnhLNYgj9Xmh7sud3X`);
    // rEjrZWyogtCxtEhf2CVizXm8DsnYqNP1Nw
    bob = Wallet.fromSecret(`snbMxLUTVDVkRArAJMxosNhdsJhJn`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    `alice pays 1000 drops of XRP to bob`,
    async () => {
      const {
        result: {
          account_data: { Balance: bobBalanceBefore },
        },
      } = await client.request({
        command: "account_info",
        account: bob.classicAddress,
        ledger_index: "validated",
      });
      const {
        result: {
          account_data: { Balance: aliceBalanceBefore },
        },
      } = await client.request({
        command: "account_info",
        account: alice.classicAddress,
        ledger_index: "validated",
      });
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

      if (txResponse.result.meta.TransactionResult !== "tesSUCCESS") {
        console.error(JSON.stringify(txResponse, null, 2));

        throw new Error(`Transaction failed`);
      }

      const [hookExecution] = txResponse.result.meta.HookExecutions as [
        HookExecution,
      ];

      const { HookReturnString, HookReturnCode } = hookExecution.HookExecution;

      expect(
        TestUtils.deserializeHexStringAsBigInt(HookReturnCode.toString()),
      ).toBe(0n);
      expect(HookReturnString).toMatch(/^[A-F0-9]{64}$/);

      await TestUtils.waitForMaybeNonExistentTx(client, HookReturnString);
      const emittedTx = await client.request({
        command: "tx",
        transaction: HookReturnString,
      });
      const emittedTxFee = emittedTx.result.Fee!;
      expect(emittedTxFee).toBeDefined();

      const {
        result: {
          account_data: { Balance: bobBalanceAfter },
        },
      } = await client.request({
        command: "account_info",
        account: bob.classicAddress,
        ledger_index: "validated",
      });
      const {
        result: {
          account_data: { Balance: aliceBalanceAfter },
        },
      } = await client.request({
        command: "account_info",
        account: alice.classicAddress,
        ledger_index: "validated",
      });

      const dropsSentToBob = 1000;
      expect(Number(bobBalanceAfter) - dropsSentToBob).toBe(
        Number(bobBalanceBefore) - Number(fee),
      );
      expect(
        Number(aliceBalanceAfter) + dropsSentToBob + Number(emittedTxFee),
      ).toBeCloseTo(Number(aliceBalanceBefore));
    },
    3 * 60_000,
  );
});
