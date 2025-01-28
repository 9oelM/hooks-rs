import { prompt, Secret } from "jsr:@cliffy/prompt@1.0.0-rc.7";
import { Logger, Network } from "../misc/mod.ts";

type XRPLSecret = string;

type HooksRsAccount = {
  password: string;
  secret: XRPLSecret;
};

type PrefundedTestnetAccount = {
  /**
   * "rBPEg6VqjrahqzAyp1UKSARuSUYLAidKGf"
   */
  address: string;
  /**
   * "ssKu7GvBV5tLfRHADFqd8EmR3ABVe"
   */
  secret: string;
  /**
   * tesSUCCESS
   */
  code: string;
  /**
   * 10000
   */
  xrp: number;
};

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
    `${Network.getRpcUrl(Network.Network.XahauTestnet)}/newcreds`,
    {
      method: `POST`,
    },
  );
  try {
    const responseJson = await response.json();
    if (
      isPrefundedTestnetAccount(responseJson)
    ) {
      if (responseJson.code === `tesSUCCESS`) {
        const secret = responseJson.secret;

        return secret;
      } else {
        Logger.log(
          `error`,
          `json deserialization worked, but transaction result code is "${responseJson.code}"`,
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
    typeof (account as PrefundedTestnetAccount).address === `string` &&
    typeof (account as PrefundedTestnetAccount).secret === `string` &&
    typeof (account as PrefundedTestnetAccount).code === `string` &&
    typeof (account as PrefundedTestnetAccount).xrp === `number`;
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
