import { Command } from "https://deno.land/x/cliffy@v1.0.0-rc.3/command/mod.ts";
import * as path from "https://deno.land/std@0.207.0/path/mod.ts";
import { Logger } from "./misc/logger.ts";
import { isMinimalCargoToml, readCargoToml } from "./misc/cargo_toml.ts";
import { TypedObjectKeys } from "./types/utils.ts";
import { copy } from "https://deno.land/std@0.207.0/fs/copy.ts";
import { DependenciesManager } from "./dependencies_manager/mod.ts";
import { HooksBuilder } from "./hooks_builder/mod.ts";

// Export for testing
export const cli = await new Command()
  .name("hooks")
  .version("0.0.1")
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
    "build",
    "Compile and preprocess a hook",
  )
  .action(async () => {
    const checksPassed = await check();
    if (checksPassed) {
      build();
    }
  })
  .command(
    `deploy`,
    `Deploy a built hook to Xahau network`,
  )
  .arguments(`[rpc: number] [hookOn: string]`)
  .action(deploy)
  .command(
    `uninstall`,
    `Uninstall all prerequisite binaries installed by 'up' command. WARNING: if you have other projects using the prerequisite binaries or if you have installed the binaries yourself in the past, those binaries will be removed and may cause you problems)`,
  )
  .action(uninstall)
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

export async function deploy(rpc?: string, hookOn?: string) {
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
