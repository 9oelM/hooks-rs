import { TypedObjectKeys } from "../types/utils.ts";
// @deno-types="../types/command_exists.d.ts"
import commandExists from "npm:command-exists";

// check inside cargo workplace
export async function installCargo() {
}

export async function checkPrerequisitesInstalled() {
  const prerequisitesInstallationStatus = {
    cargo: false,
    ["wasm-opt"]: false,
    ["hook-cleaner"]: false,
    ["wasm2wat"]: false,
    ["guard_checker"]: false,
  };

  for (const prerequisite of TypedObjectKeys(prerequisitesInstallationStatus)) {
    try {
      await commandExists(prerequisite);
      prerequisitesInstallationStatus[prerequisite] = true;
      // deno-lint-ignore no-empty
    } catch {}
  }

  return prerequisitesInstallationStatus;
}
