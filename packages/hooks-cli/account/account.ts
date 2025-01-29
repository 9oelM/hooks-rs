import { Logger, Network } from "../misc/mod.ts";

type XRPLSecret = string;

type PrefundedTestnetAccount = {
  account: {
    //     "xAddress": "XV5CC9AbwcsBYScgsjxWpe5VMooGZ8n8NMmaNuhUbqHPozq",
    xAddress: string;
    //     "secret": "snTePLtQua9MaLMsRxWuKMQVJV4XP",
    secret: string;
    //     "classicAddress": "rPSTDHkr2n9Fq7jza5Ei1nCoSMVanfLXpV",
    classicAddress: string;
    // address: 'rPgAY3v5Zag1QK1xgK2YD76drhTiAd6gCE',
    address: string;
  };
  amount: number;
  balance: number;
  trace: {
    //     "hash": "236A497826E877596EED24A9E4A59F4E47196DAB09162FA027DFF3A7487E8CD2",
    hash: string;
    //     "code": "tesSUCCESS"
    code: string;
  };
};

export async function create() {
  Logger.log(
    `info`,
    `Fetching prefunded testnet account...`,
  );
  const accountInfo = await fetchFundedTestnetAccount();
  if (!accountInfo) return;

  // write to account.json
  Logger.log(
    `info`,
    `Writing prefunded testnet account to account.json...`,
  );
  // write to account.json in current directory where cli was run
  const currentDir = Deno.cwd();

  // Check if account.json exists
  if (await Deno.stat(`${currentDir}/account.json`).catch(() => null)) {
    Logger.log(
      `info`,
      `account.json already exists. If you want to change the account used, please delete it and run this command again.`,
    );
    return;
  }

  await Deno.writeTextFile(
    `${currentDir}/account.json`,
    JSON.stringify(accountInfo, null, 2),
    {
      create: true,
    },
  );
}

export async function load() {
  const currentDir = Deno.cwd();
  const accountJsonPath = `${currentDir}/account.json`;

  if (await Deno.stat(accountJsonPath).catch(() => null)) {
    const accountJson = await Deno.readTextFile(accountJsonPath);
    const accountInfo = JSON.parse(accountJson);

    if (!isLocalAccountJson(accountInfo)) {
      Logger.log(
        `error`,
        `account.json is not in the correct format. Please check the JSON structure of account.json.`,
      );
      return;
    }

    return accountInfo;
  } else {
    Logger.log(
      `error`,
      `account.json does not exist. Please run "account create" to create a new account.`,
    );
  }
}

async function fetchFundedTestnetAccount(): Promise<
  { secret: XRPLSecret; address: string } | undefined
> {
  const response = await fetch(
    `${Network.getRpcUrl(Network.Network.XahauTestnet)}/accounts`,
    {
      method: `POST`,
    },
  );
  try {
    const responseJson = await response.json();
    if (
      isPrefundedTestnetAccount(responseJson)
    ) {
      if (responseJson.trace.code === `tesSUCCESS`) {
        Logger.log(
          `info`,
          `Successfully fetched prefunded testnet account with address "${responseJson.account.classicAddress}"`,
        );
        const secret = responseJson.account.secret;

        return { secret, address: responseJson.account.classicAddress };
      } else {
        Logger.log(
          `error`,
          `json deserialization worked, but transaction result code is "${responseJson.trace.code}"`,
        );
      }
    } else {
      Logger.log(
        `error`,
        `Could not create prefunded testnet account due to unexpected json object: "${responseJson}"`,
      );
    }
  } catch (_e) {
    const text = await response.text();
    Logger.log(
      `error`,
      `Could not create prefunded testnet account due to failed json deserialization of string: "${text}"`,
    );
  }
}

function isPrefundedTestnetAccount(
  account: unknown,
): account is PrefundedTestnetAccount {
  return typeof account === `object` &&
    account !== null &&
    `account` in account;
}

function isLocalAccountJson(
  account: unknown,
): account is { secret: XRPLSecret; address: string } {
  return typeof account === `object` &&
    account !== null &&
    `secret` in account &&
    `address` in account && typeof account.secret === `string` &&
    typeof account.address === `string`;
}
