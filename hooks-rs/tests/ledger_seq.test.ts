// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { ExecutionUtility } from "@transia/hooks-toolkit";
import { TestUtils } from "./setup";

const HOOK_NAME = "ledger_seq";

describe("ledger_seq.rs", () => {
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
    // rnYPPFFup2qHY1JXGMRgbFYxczYBSNQhGw
    alice = Wallet.fromSecret(`sszKSqi9DLUix47qKpPQUWdcxfFZe`);
    // rNz5K2a5hGSY4PaJnsPz8q297YkUVHjmpM
    bob = Wallet.fromSecret(`ssPzhndC2fpnWVs33TootivLUAJru`);

    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "gives current ledger's sequence",
    async () => {
      const tx: Invoke & Transaction = {
        TransactionType: "Invoke",
        Account: bob.classicAddress,
        Destination: alice.classicAddress,
      };
      const ledgerResponse = await client.request({
        command: `ledger`,
        ledger_index: `current`,
      });
      // A ledger index is a 32-bit unsigned integer used to identify a ledger.
      // The ledger index is sometimes known as the ledger's sequence number.
      const { ledger_index: ledgerSequence } = ledgerResponse.result.ledger;
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

      const currentLedgerSequence = TestUtils.deserializeHexStringAsBigInt(
        hookExecutions.executions[0].HookReturnCode.toString(),
      );

      expect(hookExecutions.executions[0].HookReturnString).toMatch("");
      expect(currentLedgerSequence).toBeGreaterThanOrEqual(
        BigInt(ledgerSequence),
      );
    },
    3 * 60_000,
  );
});
