// xrpl
import { Client, Invoke, Transaction, Wallet } from "@transia/xrpl";
import { ExecutionUtility } from "@transia/hooks-toolkit";
import { Faucet, TestUtils } from "./setup";

const HOOK_NAME = "xrp_payment_txn";

// const hexxx = `0000002280000000230000000024000000002E00000000201A0065D53B201B0065D53F6140000000000003E868400000000000000073210000000000000000000000000000000000000000000000000000000000000000008114E876491523BF2548562AFB3D54B2F39F2086347F83148DF77128FAA8A30EFC9B281B0AA0C9D8EC8F972DED202E000000013D00000000000000015B89B0F93A17E523D5B6950952AF94BD9F385236E294377841ACF75CDEC4A6866A5C0BFADDC8186A14B0D7FF30DD8D2FDADCE8A4A9CBB5828CBEFDB629C580C495DE5D6940741918EEBDCD729BDED5C1D72180F1BF6CD0269A77ECECCCD62B8C3CB9AA8A14E876491523BF2548562AFB3D54B2F39F2086347FE1`

// const result = decode(hexxx)

// console.log(JSON.stringify(result))

describe("xrp_payment_txn.rs", () => {
  let client: Client;
  let alice: Wallet;
  let bob: Wallet;

  beforeAll(async () => {
    const hook = await TestUtils.buildHook(HOOK_NAME);
    client = new Client("wss://hooks-testnet-v3.xrpl-labs.com", {});
    await client.connect();
    client.networkID = await client.getNetworkID();
    let [{ secret: secret0 }, { secret: secret1 }] = await Promise.all([
      (async () => {
        const acc = await Faucet.waitAndGetNewAccount()
        console.log(acc.address)
        return acc
      })(),
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
        }
      );
      if (!txResponse.result.meta) {
        throw new Error("No meta in tx response");
      }
      if (typeof txResponse.result.meta === "string") {
        throw new Error("Meta is string, not object");
      }
      const hookExecutions = await ExecutionUtility.getHookExecutionsFromMeta(
        client,
        txResponse.result.meta
      );
      if (!hookExecutions.executions[0]) {
        throw new Error(`Hook execution data is empty`);
      }

      console.log(TestUtils.deserializeHexStringAsBigInt(hookExecutions.executions[0].HookReturnCode.toString()));
      expect(hookExecutions.executions[0].HookReturnString).toMatch(
        "accept.rs: Finished."
      );
    },
    3 * 60_000
  );
});
