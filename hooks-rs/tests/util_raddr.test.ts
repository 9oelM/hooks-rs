// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { TestUtils } from "./setup";
import { HookExecution } from "@transia/xrpl/dist/npm/models/transactions/metadata";

const HOOK_NAME = "util_raddr";

describe("util_raddr.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://xahau-test.net", {});
    await client.connect();
    client.networkID = await client.getNetworkID();

    // rHvPxERqTjFTeydCkYVBjEdURf4MeW7rWN
    alice = Wallet.fromSecret(`shqPWeDjqgSp87wxidV6nYR5qzSNt`);
    // rh41GcuurC14yBr71Y48nMiriCz7oFsKM9
    bob = Wallet.fromSecret(`ssXniQYTWZWiNo2kHpetU2GcknBiQ`);
    await TestUtils.setHook(client, alice.seed!, hook);
  }, 3 * 60_000);

  afterAll(async () => {
    await client.disconnect();
  }, 10_000);

  it(
    "converts account id to an r-address",
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
      const [hookExecution] = txResponse.result.meta.HookExecutions as [
        HookExecution,
      ];

      const { HookReturnString, HookReturnCode } = hookExecution.HookExecution;

      expect(
        TestUtils.deserializeHexStringAsBigInt(HookReturnCode.toString()),
      ).toBe(0n);
      expect(HookReturnString).toMatch(
        `724c71554659474c4d4253396a46363369526b616476753363546978616452546433`
          .toUpperCase(),
      );
    },
    3 * 60_000,
  );
});
