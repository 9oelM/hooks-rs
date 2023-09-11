// xrpl
import {
  Client,
  Invoke,
  Transaction,
  Wallet,
  decodeAccountID,
} from "@transia/xrpl";
import { Faucet, TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";

const HOOK_NAME = "hook_account";

describe("hook_account.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://hooks-testnet-v3.xrpl-labs.com", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    let [{ secret: secret0 }, { secret: secret1 }] = await Promise.all([
      Faucet.waitAndGetNewAccount(),
      Faucet.waitAndGetNewAccount(),
    ]);
    alice = Wallet.fromSecret(secret0);
    bob = Wallet.fromSecret(secret1);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "accepts with the hook account address",
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
          ...rest,
          Fee: fee,
        },
        {
          wallet: bob,
        }
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

      const { HookReturnString, HookReturnCode } = hookExecution.HookExecution;

      // HookReturnString should contain 20-bytes long representation of account address in uppercase hex string.
      expect(HookReturnString).toMatch(
        decodeAccountID(alice.address).toString("hex").toUpperCase()
      );
      expect(Number(HookReturnCode)).toBe(0);
    },
    3 * 60_000
  );
});
