import { prompt, Secret } from "jsr:@cliffy/prompt@1.0.0-rc.7";
import { Logger, Network } from "../misc/mod.ts";

type XRPLSecret = string;

type HooksRsAccount = {
  password: string;
  secret: XRPLSecret;
};

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
}

export async function create(
  shouldCreatePrefundedTestnetAccount: boolean,
) {
  if (shouldCreatePrefundedTestnetAccount) {
    Logger.log(
      `info`,
      `Fetching prefunded testnet account...`,
    );
    const maybeSecret = await fetchFundedTestnetAccount();
    if (!maybeSecret) return;
    const password = await promptPassword();
    if (!password) return;
    const account: HooksRsAccount = {
      password,
      secret: maybeSecret,
    };
  }
}

export async function derive() {
  const account = await promptAccountCreationFromsecret();
  if (!account) return;
  createAccountFromSecret(account);
}

async function fetchFundedTestnetAccount(): Promise<XRPLSecret | undefined> {
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

        return secret;
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

function createNewAccount() {
}

function createAccountFromSecret(account: HooksRsAccount) {
}

function isPrefundedTestnetAccount(
  account: unknown,
): account is PrefundedTestnetAccount {
  return typeof account === `object` &&
    account !== null &&
    `account` in account
}

async function promptAccountCreationFromsecret(): Promise<
  HooksRsAccount | undefined
> {
  const result = await prompt([{
    name: `password`,
    message: `Enter a password to encrypt your account secret`,
    minLength: 1,
    type: Secret,
  }, {
    name: `secret`,
    message: `Enter your private key`,
    validate: validatesecret,
    type: Secret,
  }]);

  if (result.password === undefined || result.secret === undefined) {
    Logger.log(
      `error`,
      `Could not create account from private key due to missing password or private key`,
    );
    return;
  }

  return result as Required<typeof result>;
}

async function promptPassword(): Promise<
  string | undefined
> {
  const password = await Secret.prompt({
    message: `Enter a password to encrypt your account secret`,
    minLength: 1,
  });

  if (password === undefined) {
    Logger.log(
      `error`,
      `Could not create account due to missing password`,
    );
    return;
  }

  return password;
}

function validatesecret(secret: string): boolean {
  return true;
}
