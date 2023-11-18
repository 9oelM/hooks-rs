import { Command } from "https://deno.land/x/cliffy@v1.0.0-rc.3/command/mod.ts";
import { getRpcUrl } from "./network.ts";
import { HooksRsManager, HooksRsSetup } from "./hooks-rs-manager.ts";

import { Logger } from "./logger.ts";
import { readCargoToml, isMinimalCargoToml } from "./cargo.ts";
import { TypedObjectKeys } from "./types/utils.ts";

await new Command()
  .name("hooks")
  .version("0.0.1")
  .description("CLI for hooks-rs")
  .command(
    `up`,
    `Installs all prerequisite binaries`
  )
  .action(up)
  .command(
    "check",
    "Checks if all prerequisite binaries are installed and available in PATH",
  )
  .action(async () => {
    const prerequisitesInstallationStatus = await HooksRsSetup.checkPrerequisitesInstalled();

    let allPrerequisitesInstalled = true;
    TypedObjectKeys(prerequisitesInstallationStatus).forEach((prerequisite) => {
      if (!prerequisitesInstallationStatus[prerequisite]) {
        allPrerequisitesInstalled = false;
        Logger.log(`error`, `Could not find ${prerequisite} in PATH`);
      }
    })
    if (allPrerequisitesInstalled) {
      Logger.log(`success`, `All prerequisites are installed and available in PATH`);
    } else {
      Logger.log(`error`, `Some prerequisites are not installed or not available in PATH\n. Run "hooks up" to install them.`);
    }
  })
  .command(
    "build",
    "Compile and preprocess a hook",
  )
  .action(build)
  .command(
    `deploy`,
    `Deploy a built hook to Xahau network`,
  )
  .action(deploy)
  .parse(Deno.args);

async function up() {

}

async function build() {
  const parsedCargoToml = await readCargoToml()
  if (isMinimalCargoToml(parsedCargoToml)) {
    const { name } = parsedCargoToml.package
    
    await HooksRsManager.buildHook(name)
    Logger.log(`success`, `Successfully built hook "${name}"`)
  } else {
    Logger.log(`error`, `Invalid Cargo.toml. It must contain a [package] section with a name field.`)
  }
}

async function deploy() {

}