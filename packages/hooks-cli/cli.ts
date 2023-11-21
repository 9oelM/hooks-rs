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
  .action(deploy)
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

  for (const prerequisite of TypedObjectKeys(prerequisitesInstallationStatus)) {
    if (!prerequisitesInstallationStatus[prerequisite]) {
      await DependenciesManager.installPrerequisite(prerequisite);
    }
  }
}

export async function build() {
  const parsedCargoToml = await readCargoToml();
  if (isMinimalCargoToml(parsedCargoToml)) {
    const { name } = parsedCargoToml.package;

    await HooksBuilder.buildHook(name);
    Logger.log(`success`, `Successfully built hook "${name}"`);
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
      `Some prerequisites are not installed or not available in PATH.\n Run "hooks up" to install them.`,
    );
  }

  return allPrerequisitesInstalled;
}

export async function deploy() {
  // TODO
}
