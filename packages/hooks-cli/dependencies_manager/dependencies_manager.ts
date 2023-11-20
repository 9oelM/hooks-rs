import { Logger } from "../misc/mod.ts";
import { TypedObjectKeys } from "../types/utils.ts";
// @deno-types="../types/command_exists.d.ts"
import commandExists from "npm:command-exists";
import * as path from "https://deno.land/std@0.207.0/path/mod.ts";
import { download } from "https://deno.land/x/download@v2.0.2/mod.ts";

export interface PrerequisitesInstallationStatus {
  git: boolean;
  cargo: boolean;
  "wasm-opt": boolean;
  "hook-cleaner": boolean;
  wasm2wat: boolean;
  guard_checker: boolean;
  "wasm-pack": boolean;
}

export async function checkPrerequisitesInstalled() {
  // Do not change this order, since cargo and git are required for other installations
  // and Deno runtime will keep the order when Object.keys is called
  const prerequisitesInstallationStatus: PrerequisitesInstallationStatus = {
    git: false,
    cargo: false,
    ["wasm-opt"]: false,
    ["hook-cleaner"]: false,
    ["wasm2wat"]: false,
    ["guard_checker"]: false,
    ["wasm-pack"]: false,
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

// To simplify cross-platform installation, we use cargo to install wasm-opt
async function installWasmOpt() {
  Logger.handleOutput(
    await new Deno.Command(`cargo`, {
      args: [
        `install`,
        `wasm-opt`,
        `--locked`,
      ],
    }).output(),
  );
}

async function installCProject(
  githubRepoUrl: string,
  githubRepoName: string,
  resetToHash: string,
  binaryName: string,
) {
  const tempDirPath = await Deno.makeTempDir();
  Logger.handleOutput(
    await new Deno.Command(`git`, {
      args: [
        `clone`,
        githubRepoUrl,
      ],
      cwd: tempDirPath,
    }).output(),
  );
  Logger.handleOutput(
    await new Deno.Command(`git`, {
      args: [
        `reset`,
        `--hard`,
        resetToHash,
      ],
      cwd: path.join(tempDirPath, githubRepoName),
    }).output(),
  );
  Logger.handleOutput(
    await new Deno.Command(`make`, {
      cwd: path.join(tempDirPath, githubRepoName),
    }).output(),
  );

  switch (Deno.build.os) {
    case "windows":
      throw new Error(`Windows is not supported yet.`);
    default: {
      await Deno.chmod(
        path.join(tempDirPath, githubRepoName, binaryName),
        0o755,
      );
      await Deno.rename(
        path.join(tempDirPath, githubRepoName, binaryName),
        `/usr/local/bin/${binaryName}`,
      );
      break;
    }
  }
}

async function installHookCleaner() {
  return await installCProject(
    `https://github.com/XRPLF/hook-cleaner-c`,
    `hook-cleaner-c`,
    `b856a3614c00361f108d07379f5892e7347bb994`,
    `hook-cleaner`,
  );
}

async function installGuardChecker() {
  return await installCProject(
    `https://github.com/RichardAH/guard-checker`,
    `guard-checker`,
    `de69e8aa054d49612dda7046962003beb88c0749`,
    `guard_checker`,
  );
}

async function installWasmPack() {
  Logger.handleOutput(
    await new Deno.Command(`cargo`, {
      args: [
        `install`,
        `wasm-pack`,
        `--locked`,
      ],
    }).output(),
  );
}

async function installWasm2Wat() {
  const tempDirPath = await Deno.makeTempDir();

  switch (Deno.build.os) {
    case "windows":
      throw new Error(`Windows is not supported yet.`);
    default:
      switch (Deno.build.os) {
        case `darwin`: {
          const downloadedFile = await download(
            "https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-macos-12.tar.gz",
            {
              dir: tempDirPath,
              file: `wabt-1.0.34-macos-12.tar.gz`,
            },
          );
          Logger.handleOutput(
            await new Deno.Command(`tar`, {
              args: [
                `-xzf`,
                downloadedFile.fullPath,
              ],
            }).output(),
          );
          break;
        }
        default: {
          const downloadedFile = await download(
            // just try ubuntu for now
            "https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-ubuntu.tar.gz",
            {
              dir: tempDirPath,
              file: `wabt-1.0.34-ubuntu.tar.gz`,
            },
          );
          Logger.handleOutput(
            await new Deno.Command(`tar`, {
              args: [
                `-xzf`,
                downloadedFile.fullPath,
              ],
            }).output(),
          );
          break;
        }
      }
      await Deno.chmod(
        path.join(tempDirPath, `wabt-1.0.34`, `bin`, `wasm2wat`),
        0o755,
      );
      // the whole directory needs to be moved due to include files
      await Deno.rename(
        path.join(tempDirPath, `wabt-1.0.34`),
        `/usr/local/wabt-1.0.34`,
      );
      await Deno.symlink(
        path.join(tempDirPath, `wabt-1.0.34`, `bin`, `wasm2wat`),
        `/usr/local/bin/wasm2wat`,
      );
  }
}

export async function installPrerequisite(
  prerequisite: keyof PrerequisitesInstallationStatus,
) {
  const arch = Deno.build.arch;

  if (arch === "x86_64") {
    throw new Error(`32 bits architecture is not supported.`);
  }

  switch (prerequisite) {
    case "git":
      throw new Error(
        `You do not have git installed, but you need to install it manually. 
Refer to https://git-scm.com/book/en/v2/Getting-Started-Installing-Git for more information.`,
      );
    case "cargo":
      throw new Error(
        `You do not have cargo installed, but you need to install it manually. 
Refer to https://forge.rust-lang.org/infra/other-installation-methods.html for more information.`,
      );
    case "wasm-opt":
      await installWasmOpt();
      break;
    case "hook-cleaner":
      await installHookCleaner();
      break;
    case "wasm2wat":
      await installWasm2Wat();
      break;
    case "guard_checker":
      await installGuardChecker();
      break;
    case `wasm-pack`:
      await installWasmPack();
      break;
    default:
      throw new Error(`Unknown prerequisite ${prerequisite}.`);
  }
}
