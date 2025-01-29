import { Command } from "jsr:@cliffy/command@1.0.0-rc.7";
import * as path from "jsr:@std/path";
import commandExists from "npm:command-exists";
import { Logger } from "./misc/logger.ts";
import { isMinimalCargoToml, readCargoToml } from "./misc/cargo_toml.ts";
import { TypedObjectKeys } from "./types/utils.ts";
import { copy } from "jsr:@std/fs";
import { DependenciesManager } from "./dependencies_manager/mod.ts";
import {
  HookOnField,
  HooksBuilder,
  isXrplTransactionType,
  XrplTransactionType,
} from "./hooks_builder/mod.ts";
import { getRpcUrl } from "./misc/network.ts";
import { Network } from "./misc/mod.ts";
import { Account } from "./account/mod.ts";
import { pathExists } from "./misc/utils.ts";
import {
  Client,
  encode,
  SetHook,
  SetHookFlags,
  Transaction,
  Wallet,
} from "@transia/xrpl";
import { HookPayload } from "./types/mod.ts";

// Export for testing
export const cli = await new Command()
  .name("hooks")
  .version("0.0.2")
  .meta(`author`, `https://github.com/9oelm`)
  .meta(`project`, `https://github.com/9oelm/hooks-rs`)
  .description("CLI for hooks-rs")
  .command(
    `new`,
    `Initializes a new hooks-rs template in a new folder in the current working directory`,
  )
  .arguments("<projectName>")
  .action(newProject)
  .command(
    `up`,
    `Installs all prerequisite binaries`,
  )
  .action(up)
  .command(
    "check",
    "Checks if all prerequisite binaries are installed and available in PATH",
  )
  .action(check)
  .command(
    "account",
    "Create a new testnet account stored in account.json",
  )
  .action(async () => {
    await Account.create();
  })
  .command(
    "build",
    "Compile and preprocess a hook",
  )
  .action(async () => {
    const checksPassed = await check();
    if (checksPassed) {
      await build();
    }
  })
  .command(
    `deploy`,
    `Deploy a built hook to Xahau network`,
  )
  .option("--rpc <rpc:string>", "Websocket RPC endpoint for deployment", {
    default: getRpcUrl(Network.Network.XahauTestnet),
  })
  .option(
    "--hook-on [transactionTypes...:string]",
    "A list of HookOn fields in UPPERCASE (example: --hook-on PAYMENT TICKET_CREATE INVOKE)",
    {
      required: true,
    },
  )
  .action(async ({
    rpc,
    hookOn,
  }) => {
    const {
      hookOn: hookOnTTSet,
    } = validateDeployOptions({
      rpc,
      hookOn,
    });

    const hookOnField0xHex = new HookOnField().fromSet(new Set(hookOnTTSet))
      .toHex();

    const client = new Client(rpc, {});
    await client.connect();
    client.networkID = await client.getNetworkID();

    const account = await Account.load();
    if (!account) {
      Logger.log(
        `error`,
        `Could not load account from account.json`,
      );
      Deno.exit(1);
    }

    const hookPayload = await build();

    if (!hookPayload) {
      Logger.log(
        `error`,
        `Could not build hook.`,
      );
      Deno.exit(1);
    }

    hookPayload.HookOn = hookOnField0xHex;

    const submitResponse = await setHook(
      client,
      account.secret,
      hookPayload,
    );

    if (
      `meta` in submitResponse && typeof submitResponse.meta === `object` &&
      submitResponse.meta !== null &&
      `TransactionResult` in submitResponse.meta &&
      submitResponse.meta.TransactionResult === "tesSUCCESS"
    ) {
      Logger.log(
        `success`,
        `Successfully deployed hook`,
      );
      console.log(JSON.stringify(submitResponse, null, 2));
    } else {
      Logger.log(
        `error`,
        `Could not deploy hook: ${JSON.stringify(submitResponse)}`,
      );
      Deno.exit(1);
    }
  })
  .command(
    `uninstall`,
    `Uninstall all prerequisite binaries installed by 'up' command. 
WARNING: if you have other projects using the prerequisite binaries or if you have installed the binaries yourself in the past, those binaries will be removed and may cause you problems)`,
  )
  .action(uninstall)
  .command(
    `test`,
    `Run tests for the project`,
  )
  .action(test)
  .parse(Deno.args);

// Print help on no arguments or subcommand.
// Default behavior is to exit without printing anything.
if (cli.args.length === 0 && cli.cmd.getName() === "hooks") {
  cli.cmd.showHelp();
}

export async function newProject(_unusedOptions: void, projectName: string) {
  // run git clone
  const tempDirPath = await Deno.makeTempDir();
  const gitCloneTemplateOutput = await new Deno.Command(`git`, {
    // TODO: only clone subdirectory under hooks-rs repository
    args: [`clone`, `https://github.com/9oelm/hooks-rs`],
    cwd: tempDirPath,
  }).output();
  if (!gitCloneTemplateOutput.success) {
    Logger.log(
      `error`,
      `Could not clone hooks-rs template: ${
        new TextDecoder().decode(gitCloneTemplateOutput.stderr)
      }`,
    );
    Deno.exit(1);
  }
  const templateDirPath = path.join(
    tempDirPath,
    `hooks-rs`,
    `hooks-rs-template`,
  );
  const projectDirPath = path.join(Deno.cwd(), projectName);
  await Deno.mkdir(projectDirPath);
  await copy(templateDirPath, projectDirPath, {
    overwrite: true,
  });
  await Deno.remove(tempDirPath, { recursive: true });
  Logger.log(
    `success`,
    `Successfully created new hooks-rs project in ${projectDirPath}`,
  );

  Logger.log(`info`, `Installing npm dependencies...`);

  // check if npm is installed
  if ((await commandExists("npm"))) {
    const npmInstallOutput = await new Deno.Command(`npm`, {
      args: [`install`],
      cwd: projectDirPath,
    }).output();
    if (!npmInstallOutput.success) {
      Logger.log(
        `error`,
        `Could not install npm dependencies: ${
          new TextDecoder().decode(npmInstallOutput.stderr)
        }`,
      );
      Deno.exit(1);
    }

    Logger.log(`success`, `Successfully installed npm dependencies`);
    Logger.log(
      `info`,
      npmInstallOutput.stdout
        ? new TextDecoder().decode(npmInstallOutput.stdout)
        : "",
    );
  } else {
    Logger.log(
      `error`,
      `npm is not installed. Please install npm to continue.`,
    );
    Deno.exit(1);
  }
}

export async function up() {
  const prerequisitesInstallationStatus = await DependenciesManager
    .checkPrerequisitesInstalled();

  const installations: ReturnType<
    typeof DependenciesManager.installPrerequisite
  >[] = [];
  for (const prerequisite of TypedObjectKeys(prerequisitesInstallationStatus)) {
    if (!prerequisitesInstallationStatus[prerequisite]) {
      installations.push(DependenciesManager.installPrerequisite(prerequisite));
    }
  }

  await Promise.all(installations);
  const cargoNightlySelectedAsDefault = await DependenciesManager
    .checkCargoNightlySelectedAsDefault();
  if (prerequisitesInstallationStatus.cargo && !cargoNightlySelectedAsDefault) {
    Logger.log(
      `error`,
      `Cargo nightly is not selected as default.\nRun "rustup default nightly" to select it.`,
    );
  }

  const wasm32UnknownUnknownTargetInstalled = await DependenciesManager
    .checkRustupWasm32UnknownUnknownInstalled();

  if (
    prerequisitesInstallationStatus.cargo &&
    !wasm32UnknownUnknownTargetInstalled
  ) {
    Logger.log(
      `error`,
      `wasm32-unknown-unknown target is not installed.\nRun "rustup target add wasm32-unknown-unknown" to install it.`,
    );
  }

  await check();
}

export async function build() {
  const parsedCargoToml = await readCargoToml();
  if (isMinimalCargoToml(parsedCargoToml)) {
    const { name } = parsedCargoToml.package;

    const hookPayload = await HooksBuilder.buildHook(name);
    Logger.log(`success`, `Successfully built hook "${name}"`);

    return hookPayload;
  } else {
    Logger.log(
      `error`,
      `Invalid Cargo.toml. It must contain a [package] section with a name field.`,
    );
  }
}

export async function check() {
  const prerequisitesInstallationStatus = await DependenciesManager
    .checkPrerequisitesInstalled();

  let allPrerequisitesInstalled = true;
  TypedObjectKeys(prerequisitesInstallationStatus).forEach((prerequisite) => {
    if (!prerequisitesInstallationStatus[prerequisite]) {
      allPrerequisitesInstalled = false;
      Logger.log(`error`, `Could not find ${prerequisite} in PATH`);
    }
  });
  if (allPrerequisitesInstalled) {
    Logger.log(
      `success`,
      `All prerequisites are installed and available in PATH`,
    );
  } else {
    Logger.log(
      `error`,
      `Some prerequisites are not installed or not available in PATH.\nRun "hooks up" to install them.`,
    );
  }

  const cargoNightlySelectedAsDefault = await DependenciesManager
    .checkCargoNightlySelectedAsDefault();
  if (prerequisitesInstallationStatus.cargo && !cargoNightlySelectedAsDefault) {
    Logger.log(
      `error`,
      `Cargo nightly is not selected as default.\nRun "rustup default nightly" to select it.`,
    );
  }
  const wasm32UnknownUnknownTargetInstalled = await DependenciesManager
    .checkRustupWasm32UnknownUnknownInstalled();

  if (
    prerequisitesInstallationStatus.cargo &&
    !wasm32UnknownUnknownTargetInstalled
  ) {
    Logger.log(
      `error`,
      `wasm32-unknown-unknown target is not installed.\nRun "rustup target add wasm32-unknown-unknown" to install it.`,
    );
  }

  return allPrerequisitesInstalled && cargoNightlySelectedAsDefault &&
    wasm32UnknownUnknownTargetInstalled;
}

async function test() {
  // exists ./package.json?
  if (!(await pathExists("./package.json"))) {
    Logger.log(
      `error`,
      `package.json not found. Are you in the root of a hooks project?`,
    );
    Deno.exit(1);
  }

  // run npm test
  const process = new Deno.Command("npm", {
    args: ["test"],
    stdout: "piped",
    stderr: "piped",
  }).spawn();

  // Function to stream output
  async function streamOutput(
    reader: ReadableStreamDefaultReader<Uint8Array>,
    prefix: string,
  ) {
    const decoder = new TextDecoder();
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      console.log(`${prefix}${decoder.decode(value)}`);
    }
  }

  // Stream stdout and stderr in real time
  await Promise.all([
    streamOutput(process.stdout.getReader(), ""), // Standard output
    streamOutput(process.stderr.getReader(), "Error: "), // Standard error
  ]);

  // Wait for the process to complete
  const status = await process.status;
  if (!status.success) {
    console.error("npm test failed.");
    Deno.exit(1);
  }
}

/**
 * @throws Error if invalid options are supplied
 */
export function validateDeployOptions({
  rpc,
  hookOn,
}: {
  rpc: string;
  // Follows the auto inferred type from cliffy
  hookOn: true | string[];
}) {
  if (!Array.isArray(hookOn) || hookOn.length === 0) {
    throw new Error(
      `HookOn field must be a list of transaction tyes. For example:
--hook-on PAYMENT TICKET_CREATE INVOKE`,
    );
  }

  const invalidTransactionTypes = hookOn.filter(
    (hookOnField) => !isXrplTransactionType(hookOnField),
  ).join(`, `);

  if (invalidTransactionTypes.length > 0) {
    throw new Error(
      `Invalid transaction types supplied for HookOn field: ${invalidTransactionTypes}`,
    );
  }

  const hookOnSet = new Set<keyof typeof XrplTransactionType>(
    // we checked that all elements are valid transaction types above
    hookOn as (keyof typeof XrplTransactionType)[],
  );

  // Will throw if not a valid URL
  const rpcUrl = new URL(rpc);

  return {
    hookOn: hookOnSet,
    rpc: rpcUrl,
  };
}

export async function uninstall() {
  const prerequisitesInstallationStatus = await DependenciesManager
    .checkPrerequisitesInstalled();

  const uninstallations: ReturnType<
    typeof DependenciesManager.uninstallPrerequisite
  >[] = [];
  for (const prerequisite of TypedObjectKeys(prerequisitesInstallationStatus)) {
    if (prerequisitesInstallationStatus[prerequisite]) {
      uninstallations.push(
        DependenciesManager.uninstallPrerequisite(prerequisite),
      );
    }
  }

  await Promise.all(uninstallations);
}

async function setHook(client: Client, secret: string, hook: HookPayload) {
  const wallet = Wallet.fromSecret(secret);

  const tx: SetHook = {
    TransactionType: `SetHook`,
    Account: wallet.address,
    Hooks: [{ Hook: hook }],
    Flags: SetHookFlags.hsfOverride | SetHookFlags.hsfNSDelete,
  };

  const fee = await getTransactionFee(client, tx);
  tx.Fee = fee;

  const submitResponse = await submitAndWaitWithRetries(
    client,
    tx,
    {
      wallet,
      failHard: true,
      autofill: true,
    },
  );

  return submitResponse;
}

async function getTransactionFee(
  client: Client,
  transaction: Transaction,
): Promise<string> {
  const copyTx = JSON.parse(JSON.stringify(transaction));
  copyTx.Fee = `0`;
  copyTx.SigningPubKey = ``;

  const preparedTx = await client.autofill(copyTx);

  const tx_blob = encode(preparedTx);

  const result = await getFeeEstimateXrp(client, tx_blob);

  return result;
}

async function submitAndWaitWithRetries(
  client: Client,
  ...params: Parameters<Client["submitAndWait"]>
) {
  let tries = 0;
  while (tries < 3) {
    try {
      const result = await client.submitAndWait(...params);
      return result;
    } catch (e) {
      console.error(`${e} - retrying...`);
      await new Promise((resolve) => setTimeout(resolve, 1000));
      tries++;
      continue;
    }
  }

  throw new Error(`Could not submit transaction after ${tries} tries`);
}
